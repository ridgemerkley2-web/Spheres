// Exercise the exact patched dependency source in normal workspace CI, without
// activating tiny_http's unrelated example and benchmark development crates.
#[allow(dead_code)]
#[path = "../../vendor/tiny_http/src/util/task_pool.rs"]
mod transport_task_pool;
