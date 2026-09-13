use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// Manages a collection of threads.
///
/// A new thread is created every time all the existing threads are full.
/// Any idle thread will automatically die after a few seconds.
pub struct TaskPool {
    sharing: Arc<Sharing>,
}

struct Sharing {
    // list of the tasks to be done by worker threads
    todo: Mutex<VecDeque<Box<dyn FnMut() + Send>>>,

    // condvar that will be notified whenever a task is added to `todo`
    condvar: Condvar,

    // number of total worker threads running
    active_tasks: AtomicUsize,

    // number of idle worker threads
    waiting_tasks: AtomicUsize,
}

/// Minimum number of active threads.
static MIN_THREADS: usize = 4;

struct Registration<'a> {
    nb: &'a AtomicUsize,
}

impl<'a> Registration<'a> {
    fn new(nb: &'a AtomicUsize) -> Registration<'a> {
        nb.fetch_add(1, Ordering::Release);
        Registration { nb }
    }
}

impl<'a> Drop for Registration<'a> {
    fn drop(&mut self) {
        self.nb.fetch_sub(1, Ordering::Release);
    }
}

impl TaskPool {
    pub fn new() -> TaskPool {
        let pool = TaskPool {
            sharing: Arc::new(Sharing {
                todo: Mutex::new(VecDeque::new()),
                condvar: Condvar::new(),
                active_tasks: AtomicUsize::new(0),
                waiting_tasks: AtomicUsize::new(0),
            }),
        };

        for _ in 0..MIN_THREADS {
            pool.add_thread(None)
        }

        pool
    }

    /// Executes a function in a thread.
    /// If no thread is available, spawns a new one.
    pub fn spawn(&self, code: Box<dyn FnMut() + Send>) {
        let mut queue = self.sharing.todo.lock().unwrap();
        self.spawn_locked(code, &mut queue);
    }

    // Fixture-only seam: preserve the original admission body while allowing
    // a deterministic burst under its existing queue lock.
    fn spawn_locked(&self, code: Box<dyn FnMut() + Send>, queue: &mut VecDeque<Box<dyn FnMut() + Send>>) {
        if self.sharing.waiting_tasks.load(Ordering::Acquire) == 0 {
            self.add_thread(Some(code));
        } else {
            queue.push_back(code);
            self.sharing.condvar.notify_one();
        }
    }

    fn add_thread(&self, initial_fn: Option<Box<dyn FnMut() + Send>>) {
        let sharing = self.sharing.clone();

        thread::spawn(move || {
            let sharing = sharing;
            let _active_guard = Registration::new(&sharing.active_tasks);

            if let Some(mut f) = initial_fn {
                f();
            }

            loop {
                let mut task: Box<dyn FnMut() + Send> = {
                    let mut todo = sharing.todo.lock().unwrap();

                    let task;
                    loop {
                        if let Some(poped_task) = todo.pop_front() {
                            task = poped_task;
                            break;
                        }
                        let _waiting_guard = Registration::new(&sharing.waiting_tasks);

                        let received =
                            if sharing.active_tasks.load(Ordering::Acquire) <= MIN_THREADS {
                                todo = sharing.condvar.wait(todo).unwrap();
                                true
                            } else {
                                let (new_lock, waitres) = sharing
                                    .condvar
                                    .wait_timeout(todo, Duration::from_millis(5000))
                                    .unwrap();
                                todo = new_lock;
                                !waitres.timed_out()
                            };

                        if !received && todo.is_empty() {
                            return;
                        }
                    }

                    task
                };

                task();
            }
        });
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.sharing
            .active_tasks
            .store(999_999_999, Ordering::Release);
        self.sharing.condvar.notify_all();
    }
}

#[cfg(test)]
mod s09_regression {
    use super::*;
    use std::sync::mpsc;
    use std::time::Instant;

    #[test]
    fn s09_a_burst_must_not_strand_connections_behind_idle_keepalive_workers() {
        let pool = TaskPool::new();
        let idle_deadline = Instant::now() + Duration::from_secs(2);
        while pool.sharing.waiting_tasks.load(Ordering::Acquire) != MIN_THREADS {
            assert!(Instant::now() < idle_deadline, "initial worker idle precondition failed");
            thread::yield_now();
        }
        let release = Arc::new((Mutex::new(false), Condvar::new()));
        let (started_tx, started_rx) = mpsc::channel();
        let (finished_tx, finished_rx) = mpsc::channel();
        let count = MIN_THREADS * 2;
        // Real spawn holds this exact mutex during admission. By holding it
        // across the burst we force the legitimate scheduling interleaving in
        // which notified workers have not yet reacquired it and decremented
        // waiting_tasks. Each admitted task models a persistent connection
        // waiting for its next request, and is explicitly released below.
        {
            let mut queue = pool.sharing.todo.lock().unwrap();
            assert_eq!(pool.sharing.waiting_tasks.load(Ordering::Acquire), MIN_THREADS);
            for id in 0..count {
                let release = release.clone();
                let started = started_tx.clone();
                let finished = finished_tx.clone();
                pool.spawn_locked(Box::new(move || {
                    started.send(id).unwrap();
                    let (mutex, condvar) = &*release;
                    let mut permitted = mutex.lock().unwrap();
                    while !*permitted { permitted = condvar.wait(permitted).unwrap(); }
                    finished.send(id).unwrap();
                }), &mut queue);
            }
        }
        let deadline = Instant::now() + Duration::from_secs(1);
        let mut before_release = Vec::new();
        while before_release.len() < count {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() { break; }
            match started_rx.recv_timeout(remaining) {
                Ok(id) => before_release.push(id),
                Err(_) => break,
            }
        }
        // Always unblock workers before asserting, including the failing
        // baseline; no deliberately blocked connections survive this test.
        {
            let (mutex, condvar) = &*release;
            *mutex.lock().unwrap() = true;
            condvar.notify_all();
        }
        let cleanup_deadline = Instant::now() + Duration::from_secs(2);
        let mut completed = Vec::new();
        while completed.len() < count {
            let remaining = cleanup_deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() { break; }
            match finished_rx.recv_timeout(remaining) {
                Ok(id) => completed.push(id),
                Err(_) => break,
            }
        }
        before_release.sort(); before_release.dedup();
        completed.sort(); completed.dedup();
        println!("initial_waiters={} burst={} started_before_release={} completed_after_release={}", MIN_THREADS, count, before_release.len(), completed.len());
        assert_eq!(completed, (0..count).collect::<Vec<_>>(), "all worker tasks must finish during cleanup");
        assert_eq!(before_release.len(), count, "queued connections were stranded behind occupied keep-alive workers");
    }
}
