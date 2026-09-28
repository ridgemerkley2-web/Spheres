//! Inspect the actual release executable without creating a campaign or server.
use std::{
    fs,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

#[test]
fn build_identity_exits_without_touching_the_save_directory() {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let directory = std::env::temp_dir().join(format!(
        "spheres-build-identity-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&directory).unwrap();
    let mut process = Command::new(env!("CARGO_BIN_EXE_spheres-web"))
        .args(["--build-info", "--port", "0"])
        .current_dir(&directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if process.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            process.kill().unwrap();
            let _ = process.wait();
            panic!("--build-info must print identity and exit before starting the game");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(
        output.stderr.is_empty(),
        "Identity inspection must not start a listener or print a launch error"
    );
    let identity: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(identity["full_revision"], env!("SPHERES_FULL_REVISION"));
    assert_eq!(identity["revision"], env!("SPHERES_REVISION"));
    assert_eq!(identity["target_os"], std::env::consts::OS);
    assert_eq!(identity["target_arch"], std::env::consts::ARCH);
    assert_eq!(identity["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(
        fs::canonicalize(identity["save_directory"].as_str().unwrap()).unwrap(),
        fs::canonicalize(&directory).unwrap()
    );
    assert_eq!(
        fs::read_dir(&directory).unwrap().count(),
        0,
        "Identity inspection wrote to the save directory"
    );
    fs::remove_dir(&directory).unwrap();
}
