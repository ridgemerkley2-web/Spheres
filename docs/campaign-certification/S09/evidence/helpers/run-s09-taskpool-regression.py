import datetime, difflib, hashlib, json, pathlib, subprocess, tempfile

BASE = pathlib.Path(__file__).resolve().parent
SOURCE = pathlib.Path(r'C:\Users\ridge\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tiny_http-0.12.0\src\util\task_pool.rs')
OUTROOT = BASE / 'evidence' / 'S09-startup-taskpool-regression'
OUTROOT.mkdir(parents=True, exist_ok=True)
OUT = pathlib.Path(tempfile.mkdtemp(prefix='pool-', dir=OUTROOT))
original = SOURCE.read_bytes()
source = original.decode('utf-8').replace('\r\n', '\n')
needle = '''        let mut queue = self.sharing.todo.lock().unwrap();

        if self.sharing.waiting_tasks.load(Ordering::Acquire) == 0 {'''
replacement = '''        let mut queue = self.sharing.todo.lock().unwrap();
        self.spawn_locked(code, &mut queue);
    }

    // Fixture-only seam: preserve the original admission body while allowing
    // a deterministic burst under its existing queue lock.
    fn spawn_locked(&self, code: Box<dyn FnMut() + Send>, queue: &mut VecDeque<Box<dyn FnMut() + Send>>) {
        if self.sharing.waiting_tasks.load(Ordering::Acquire) == 0 {'''
assert source.count(needle) == 1
baseline = source.replace(needle, replacement)
assert baseline.count('if self.sharing.waiting_tasks.load(Ordering::Acquire) == 0 {') == 1
corrected = baseline.replace('if self.sharing.waiting_tasks.load(Ordering::Acquire) == 0 {', 'if self.sharing.waiting_tasks.load(Ordering::Acquire) <= queue.len() {')
tests = r'''
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
'''

def digest(data): return hashlib.sha256(data).hexdigest()
def run(args, filename, timeout):
    result = subprocess.run(args, cwd=OUT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=timeout)
    (OUT / filename).write_bytes(result.stdout)
    return {'command': args, 'exit_code': result.returncode, 'log': filename,
            'log_sha256': digest(result.stdout), 'output': result.stdout.decode('utf-8', errors='replace')}

proof = {'kind': 'deterministic standalone tiny_http TaskPool admission regression',
         'qualification': False, 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
         'upstream_source': str(SOURCE), 'upstream_source_sha256': digest(original),
         'fixture_seam': 'Only extract the existing queue-locked admission body to spawn_locked; the fixture locks across a burst to force the allowed pre-wakeup scheduling interleaving. Both variants share this seam and test.',
         'candidate_change': 'waiting_tasks == 0 -> waiting_tasks <= queue.len()', 'results': {}}
(OUT / 'upstream-task-pool.rs').write_bytes(original)
(OUT / 'fixture-seam.patch').write_text(''.join(difflib.unified_diff(source.splitlines(True), baseline.splitlines(True), fromfile='upstream', tofile='baseline-fixture')), encoding='utf-8')
(OUT / 'candidate-condition.patch').write_text(''.join(difflib.unified_diff(baseline.splitlines(True), corrected.splitlines(True), fromfile='baseline-fixture', tofile='corrected-fixture')), encoding='utf-8')
for name, text in [('baseline', baseline), ('corrected', corrected)]:
    source_path = OUT / (name + '.rs'); source_path.write_text(text + tests, encoding='utf-8')
    executable = OUT / (name + '.exe')
    compiled = run(['rustc', '--edition=2021', '--test', str(source_path), '-o', str(executable)], name + '-compile.log', 30)
    assert compiled['exit_code'] == 0, compiled['output']
    tested = run([str(executable), '--nocapture', '--test-threads=1'], name + '-test.log', 10)
    proof['results'][name] = {'source_sha256': digest(source_path.read_bytes()), 'compile': compiled, 'test': tested}
baseline_run = proof['results']['baseline']['test']; corrected_run = proof['results']['corrected']['test']
proof['baseline_reproduced'] = baseline_run['exit_code'] == 101 and 'initial_waiters=4 burst=8 started_before_release=4 completed_after_release=8' in baseline_run['output']
proof['corrected_passed'] = corrected_run['exit_code'] == 0 and 'initial_waiters=4 burst=8 started_before_release=8 completed_after_release=8' in corrected_run['output']
proof['passed'] = proof['baseline_reproduced'] and proof['corrected_passed']
proof['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
(OUT / 'result.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'out':str(OUT), 'baseline_reproduced':proof['baseline_reproduced'], 'corrected_passed':proof['corrected_passed'], 'passed':proof['passed']}))
assert proof['passed'], 'The required failing baseline and passing corrected condition were not both demonstrated'
