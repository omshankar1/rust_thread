use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub fn sync_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // Arc: `Arc<T>` provides shared ownership of a value of type `T`.
    //    - wrap a value we're trying to share and
    //    - act as a pointer to it.
    // Arc keeps track of all of the copies of the pointer and
    // as soon as the last pointer goes out of scope it can safely
    // free the memory

    // Mutex wraps the data structure to be gaurded
    //  1. Lock to get the MutexGuard
    //  2. Mutate the data gaurded
    //  3. Unlocking happends when scope end
    let vec: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![0, 1]));

    // ******************* Thread 1 *******************
    // Arc clone: The data is common but ref count increases atomically)
    let v1 = Arc::clone(&vec);
    let handle1 = thread::spawn(move || {
        let mut v: MutexGuard<Vec<i32>> = v1.lock().unwrap();
        v.push(2);
        v.push(4);
        // println!("Sync Mutex thr1: {:?}", v);
    });
    handles.push(handle1);

    // ******************* Thread 2 *******************
    let v2 = Arc::clone(&vec);
    let handle2 = thread::spawn(move || {
        // Create a local scope
        {
            let mut v = v2.lock().unwrap();
            v.push(3);
            v.push(5);
            // println!("Sync Mutex thr2: {:?}", v);
        } // mutex unlocks at the end of this scope

        // Do some work which need not be under critical section
    });

    // Join the threads
    handles.push(handle2);
    for handle in handles {
        handle.join()?;
    }
    Ok(())
}
// std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
