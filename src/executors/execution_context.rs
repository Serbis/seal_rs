use std::sync::{Mutex, Arc};

pub type ExecutorTask = Arc<Mutex<Fn() -> () + Send>>;

pub trait ExecutionContext {
    fn register_for_execution(self: &mut Self, bid: i32, f: ExecutorTask);

    fn run(self: &mut Self);
}