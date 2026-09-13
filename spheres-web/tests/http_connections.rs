// Compile the shipped pool source so its scheduling regression is part of the
// game's ordinary workspace checks, without running upstream example programs.
#[allow(dead_code)]
#[path = "../vendor/tiny_http/src/util/task_pool.rs"]
mod connection_pool;
