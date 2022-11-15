#![allow(dead_code, unused_variables, unused_doc_comments)]
use std::sync::{Mutex, MutexGuard};
use std::thread;

pub fn sync_thread() -> thread::Result<()> {
    let vec: Mutex<Vec<i32>> = Mutex::new(vec![0, 1]);

    /// The threads are joined before the scoped closure goes out of scope,
    /// eliminating the concern of vec going out of scope earlier than desired
    ///   pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    ///   where
    ///       F: FnOnce() -> T + Send + 'scope,
    ///       T: Send + 'scope,
    thread::scope(|s| {
        s.spawn(|| {
            let mut v: MutexGuard<Vec<i32>> = vec.lock().unwrap();
            v.push(2);
            v.push(4);
        });
        s.spawn(|| {
            let mut v = vec.lock().unwrap();
            v.push(3);
            v.push(5);
        });
        s.spawn(|| {
            let mut v = vec.lock().unwrap();
            v.push(8);
            v.push(9);
        });
    });
    println!("Sync Mutex thr1: {:?}", vec);
    Ok(())
}
