use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub fn sync_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // Arc enables to safely share a value between multiple threads(solves 'static reqt of thread::spawn)
    // Mutex is a wrapper over another type, which allows safe mutability across threads

    let wrapped_arc_mutex: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![0, 1]));

    // ******************* Thread 1 *******************
    // Arc clone: The data is common but ref count increases atomically)
    let wrapped_arc_mutex_clone1 = Arc::clone(&wrapped_arc_mutex);
    let handle1 = thread::spawn(move || {
        // Create a local scope
        {
            // Deref for Arc: (*wrapped_arc_mutex_clone1).lock().unwrap()
            let mut v: MutexGuard<Vec<i32>> = wrapped_arc_mutex_clone1.lock().unwrap();
            (*v).push(2); // Deref for MutexGuard: (*v).push(2)
            v.push(4);
            // println!("Sync Mutex thr1: {:?}", v);
        } // mutex unlocks at the end of this scope(Drop Trait)

        // Do some work which need not be under critical section
    });
    handles.push(handle1);

    // ******************* Thread 2 *******************
    let wrapped_arc_mutex_clone2 = Arc::clone(&wrapped_arc_mutex);
    let handle2 = thread::spawn(move || {
        let mut v = wrapped_arc_mutex_clone2.lock().unwrap();
        v.push(3);
        v.push(5);
        // println!("Sync Mutex thr2: {:?}", v);
    });

    // Join the threads
    handles.push(handle2);
    for handle in handles {
        handle.join()?;
    }
    Ok(())
}
