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

        if self.sharing.waiting_tasks.load(Ordering::Acquire) <= queue.len() {
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
mod queued_connection_tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn an_idle_worker_reserved_by_a_queued_connection_cannot_strand_the_next_connection() {
        // Pause one real worker during its registered idle/wakeup phase. This
        // forces the burst-accept interleaving without hoping that the operating
        // system happens to schedule two spawn calls before a worker wakes.
        let pool = TaskPool {
            sharing: Arc::new(Sharing {
                todo: Mutex::new(VecDeque::new()),
                condvar: Condvar::new(),
                active_tasks: AtomicUsize::new(0),
                waiting_tasks: AtomicUsize::new(0),
            }),
        };
        let (parked_tx, parked_rx) = mpsc::channel();
        let (wake_tx, wake_rx) = mpsc::channel();
        let sharing = pool.sharing.clone();
        let worker = thread::spawn(move || {
            let _active = Registration::new(&sharing.active_tasks);
            {
                let _waiting = Registration::new(&sharing.waiting_tasks);
                parked_tx.send(()).unwrap();
                wake_rx.recv().unwrap();
            }
            loop {
                let task = sharing.todo.lock().unwrap().pop_front();
                match task {
                    Some(mut task) => task(),
                    None => break,
                }
            }
        });
        parked_rx.recv_timeout(Duration::from_secs(5)).unwrap();

        let connection_closed = Arc::new((Mutex::new(false), Condvar::new()));
        let held_connection = connection_closed.clone();
        pool.spawn(Box::new(move || {
            let (closed, changed) = &*held_connection;
            let mut closed = closed.lock().unwrap();
            while !*closed {
                closed = changed.wait(closed).unwrap();
            }
        }));
        assert_eq!(pool.sharing.todo.lock().unwrap().len(), 1);

        // The first queued keep-alive connection already owns the idle slot.
        // A second complete request must get another worker before that first
        // connection closes, even though its owner has not yet resumed.
        let (answered_tx, answered_rx) = mpsc::channel();
        pool.spawn(Box::new(move || answered_tx.send(()).unwrap()));
        let answered_before_close = answered_rx.recv_timeout(Duration::from_secs(5)).is_ok();

        // Release and join the controlled worker before asserting, including on
        // the expected pre-fix failure. No stranded fixture survives the test.
        *connection_closed.0.lock().unwrap() = true;
        connection_closed.1.notify_all();
        wake_tx.send(()).unwrap();
        worker.join().unwrap();
        assert!(answered_before_close,
            "the second connection was stranded until an unrelated keep-alive client closed");
    }
}
