#![allow(dead_code, unused_variables, unused_doc_comments)]
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

/*
////////////////////////////////////////////////////////
/// Basic threading 1: Printing a vector in a thread
pub fn basic_thread1() {
    let v = vec![0, 1];

    /// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    /// where
    ///     F: FnOnce() -> T,  // All captures to be 'moved' inside
    ///     F: Send + 'static,

    let handle1 = thread::spawn(|| {
        // println! only needs a shared reference
        println!("Vector: {:?}", v);
    });
    handle1.join().unwrap();
}
*/

/*
////////////////////////////////////////////////////////
/// Basic threading 2: Printing Vector v in 2 spawned threads
pub fn basic_thread2() {
    let v = vec![0, 1];

    let handle1 = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    let handle2 = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    // handle2.join().unwrap();
    handle1.join().unwrap();
}
*/

////////////////////////////////////////////////////////
/// Basic threading 3: Usage of Arc to extend the lifetime of vector
pub fn basic_thread3() {
    let v = vec![0, 1];
    let arc_v = Arc::new(v);

    let arc_clone1 = Arc::clone(&arc_v);
    let handle1 = thread::spawn(move || {
        println!("Vector: {:?}", arc_clone1);
    });

    let handle2 = thread::spawn(move || {
        println!("Vector: {:?}", arc_v);
    });

    handle2.join().unwrap();
    handle1.join().unwrap();
}

////////////////////////////////////////////////////////
/// Basic threading 4: Mutating the vector concurrently
pub fn basic_thread4() {
    /// Starting the vec 'v' with [0, 1]
    let mut v = vec![0, 1];

    let handle1 = thread::spawn(move || {
        v.push(1);
        v.push(2);
        println!("Vec: {:?}", v);
        v
    });

    // v.push(3); // Borrow of moved value: v

    // let handle2 = thread::spawn(move || {
    //     // Closure may outlive v
    //     v.push(4);
    //     v.push(5);
    //     println!("Vec: {:?}", v);
    //     v
    // });
    // handle2.join();

    handle1.join().unwrap();
}

/*
////////////////////////////////////////////////////////
/// Basic threading 5: Mutating the vector concurrently
pub fn basic_thread5() {
    /// Starting the vec 'v' with [0, 1]
    let mut v = vec![0, 1];
    let arc = Arc::new(v);

    let arc_clone1 = Arc::clone(&arc);
    let handle1 = thread::spawn(move || {
        arc_clone1.push(1);
        arc_clone1.push(2);
        println!("Vector: {:?}", arc_clone1);
    });

    let arc_clone2 = Arc::clone(&arc);
    let handle2 = thread::spawn(move || {
        arc_clone2.push(3);
        arc_clone2.push(4);
        println!("Vector: {:?}", arc_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
*/

/*
////////////////////////////////////////////////////////
/// Basic threading 5: Mutating the vector concurrently
pub fn basic_thread5() {
    /// Starting the vec 'v' with [0, 1]
    let v = vec![0, 1];
    let mutex_v = Mutex::new(v);

    let handle1 = thread::spawn(|| {
        let mut v = mutex_v.lock().unwrap();
        v.push(1);
        v.push(2);
        println!("Vector: {:?}", v);
    });

    // let handle2 = thread::spawn(move || {
    //     let v = mutex_v.lock().unwrap();
    //     v.push(1);
    //     v.push(2);
    //     println!("Vector: {:?}", v);
    // });
}
*/

////////////////////////////////////////////////////////
/// Basic threading 5: Mutating the vector concurrently
pub fn sync_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // Recap:
    //    Arc enables to safely share a value between multiple threads(solves 'static reqt of thread::spawn)
    //    Mutex is a wrapper over the data(other type) to be gaurded, which allows safe mutability across threads

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

pub fn scoped_thread() -> thread::Result<i32> {
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
    // Collecting the sum of the vec elements into res
    let res: i32 = {
        let vector = vec.lock().unwrap();
        vector.iter().sum()
    };
    Ok(res)
}

////////////////////////////////////////////////////////
/// pub type Result<T> = crate::result::Result<T, Box<dyn Any + Send + 'static>>;
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
