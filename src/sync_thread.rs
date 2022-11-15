use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub fn sync_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // Arc enables to safely share a value between multiple threads(solves 'static reqt of thread::spawn)
    // Mutex is a wrapper over the data(other type) to be gaurded, which allows safe mutability across threads

    // vector to be mutated atomically across threads
    let vector = vec![0, 1];
    let wrapped_arc_mutex: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vector));

    // ******************* Thread 1 *******************
    let wrapped_arc_mutex_clone1 = Arc::clone(&wrapped_arc_mutex);
    let handle1 = thread::spawn(move || {
        // Create a local scope
        {
            // Deref for Arc to yield inner value: (*wrapped_arc_mutex_clone1).lock().unwrap()
            let mut v: MutexGuard<Vec<i32>> = wrapped_arc_mutex_clone1.lock().unwrap();

            (*v).push(2); // Deref Trait for MutexGuard to yield inner value: (*v).push(2)
            v.push(4);
        } // mutex unlocks at the end of this scope(Drop Trait for MutexGuard )

        // Do some non critical work which need not be under critical section
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
