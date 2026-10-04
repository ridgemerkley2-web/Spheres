fn main() {
    stackprobe::report("plain fn main");
    let h = std::thread::spawn(|| stackprobe::report("std::thread::spawn default"));
    h.join().unwrap();
}
