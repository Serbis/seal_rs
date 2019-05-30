use std::sync::{Mutex, Arc, Condvar};

pub type TSafe<T> = Arc<Mutex<T>>;

#[macro_export]
macro_rules! tsafe {
    ($l:expr) => {
        {
           Arc::new(Mutex::new($l))
        }
    };
}