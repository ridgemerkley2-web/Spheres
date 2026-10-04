//! Throwaway probe: which thread runs a libtest test, and how large is its stack?
pub fn report(label: &str) {
    let local = 0u8;
    let addr = &local as *const u8 as usize;
    let me = std::thread::current();
    let tself = std::fs::read_link("/proc/thread-self").map(|p| p.display().to_string()).unwrap_or_default();
    let pid = std::process::id();
    let tid: u32 = tself.rsplit('/').next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let maps = std::fs::read_to_string("/proc/self/maps").unwrap_or_default();
    let mut region = String::new();
    for line in maps.lines() {
        let range = line.split_whitespace().next().unwrap_or("");
        if let Some((a, b)) = range.split_once('-') {
            let (a, b) = (usize::from_str_radix(a, 16).unwrap(), usize::from_str_radix(b, 16).unwrap());
            if a <= addr && addr < b {
                region = format!("{line} size_kib={}", (b - a) / 1024);
            }
        }
    }
    // rlimit of the main thread stack for comparison
    let limits = std::fs::read_to_string("/proc/self/limits").unwrap_or_default();
    let stack_limit = limits.lines().find(|l| l.starts_with("Max stack size")).unwrap_or("").to_string();
    println!("[{label}] thread_name={:?} pid={pid} tid={tid} is_main_thread={} RUST_MIN_STACK={:?}\n  stack_mapping: {region}\n  {stack_limit}",
        me.name(), tid == pid, std::env::var("RUST_MIN_STACK").ok());
}

/// Recurse with a fixed ~1 KiB frame and report the deepest level reached before
/// a caller-supplied limit (never overflows: limit is chosen below stack size).
#[inline(never)]
pub fn depth(n: usize, limit: usize) -> usize {
    let buf = std::hint::black_box([n as u8; 1024]);
    if n >= limit { return n + buf[0] as usize * 0; }
    depth(n + 1, limit) + std::hint::black_box(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn where_does_this_test_run() {
        super::report("libtest #[test]");
    }
    #[test]
    #[ignore]
    fn overflow_probe() {
        // ~1.1 KiB/frame; 4000 frames ~= 4.4 MiB: overflows a 2 MiB thread, fits 8 MiB.
        let d = super::depth(0, 4000);
        println!("[overflow_probe] reached depth {d} without overflow");
    }
}
