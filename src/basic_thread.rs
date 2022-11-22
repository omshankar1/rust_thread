#![allow(dead_code, unused_variables, unused_doc_comments, unused_imports)]
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time;

////////////////////////////////////////////////////////
/// Basic threading 1:
///     - Print a vector in a thread
///     pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
///         F: FnOnce() -> T,  // All captures to be 'moved' inside
///         F: Send + 'static,
///
/*
pub fn basic_thread1() {
    let v = vec![0, 1];

    let handle1 = thread::spawn(|| {
        println!("Vector: {:?}", v); // println! only needs a shared reference
    });
    handle1.join().unwrap();

    // Issue: v is borrowed(Fn) but spawn needs FnOnce
    //       `move` converts any variables captured by reference
    //        or mutable reference to variables captured by value.
}
*/
////////////////////////////////////////////////////////
/// Basic threading 2:
///     - Print a vector concurrently from couple of threads
///     - need for a shared reference kind of smart pointer
///
/*
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

    // Issue: no vector 'v' available to be moved in this closure
    //        Need a way to share v across the 2 threads
}
*/
////////////////////////////////////////////////////////
/// Basic threading 3: Usage of Arc to extend the lifetime of vector and sharing
///                    multiple references
///     - Print a vector concurrently from couple of threads

pub fn basic_thread3() {
    let v = vec![0, 1];
    let arc_v = Arc::new(v);

    let arc_clone1 = Arc::clone(&arc_v);
    let handle1 = thread::spawn(move || {
        println!("basic_thread3 Vector: {:?}", arc_clone1);
    });

    let arc_clone2 = Arc::clone(&arc_v);
    let handle2 = thread::spawn(move || {
        println!("basic_thread3 Vector: {:?}", arc_clone2);
    });

    handle2.join().unwrap();
    handle1.join().unwrap();
}

/////////////////////////////////////////////////////////////
/// Basic threading 4 'Arc': Mutating the vector concurrently
///
/*
pub fn basic_thread4() {
    /// Starting the vec 'v' with [0, 1]
    let mut v = vec![0, 1];
    let arc = Arc::new(v);

    let arc_clone1 = Arc::clone(&arc);
    let handle1 = thread::spawn(move || {
        arc_clone1.push(1);
        arc_clone1.push(2);
        println!("Vector: {:?}", arc_clone1);
    });
    handle1.join().unwrap();

    // Issue: Can't wrap a &mut T by an Arc (fn clone(&self) -> Arc<T>)
    // Can only pass shared borrow(immutable data) across using Arc
}
*/
/////////////////////////////////////////////////////////////
/// Basic threading 5: Mutate vector v by wrapping inside Mutex
///
pub fn basic_thread5() {
    /// Starting the vec 'v' with [0, 1]
    let v = vec![0, 1];
    let mutex_v = Mutex::new(v);

    let handle1 = thread::spawn(move || {
        let mut v = mutex_v.lock().unwrap();
        v.push(1);
        v.push(2);
        println!("basic_thread5 Vector: {:?}", v);
    });
    handle1.join().unwrap();

    // let handle2 = thread::spawn(move || {
    //     let v = mutex_v.lock().unwrap();
    //     v.push(1);
    //     v.push(2);
    //     println!("Vector: {:?}", v);
    // });
    // handle2.join().unwrap();

    // Issue: no mutex_v left for thread2.

    // Note:
    //   Interior Mutability:
    //      Lock doesn't need a mutable &mut self, tricks compiler to
    //      think we're sharing only immutable data
    //      pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
}

////////////////////////////////////////////////////////
/// Complete example
pub fn sync_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // Recap:
    //    Arc enables to safely share a value between multiple
    //      threads(solves 'static reqt of thread::spawn)
    //    Mutex is a wrapper over the data(other type) to be gaurded,
    //      which allows safe mutability across threads

    // vector to be mutated atomically across threads
    let vector = vec![0, 1];
    let wrapped_arc_mutex: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vector));

    // ******************* Thread 1 *******************
    let wrapped_arc_mutex_clone1 = Arc::clone(&wrapped_arc_mutex);
    let handle1 = thread::spawn(move || {
        // Create a local scope to allow minimal critical section
        {
            // Deref for Arc to yield inner value: (*wrapped_arc_mutex_clone1).lock().unwrap()
            let mut v: MutexGuard<Vec<i32>> = wrapped_arc_mutex_clone1.lock().unwrap();

            (*v).push(2); // Deref Trait for MutexGuard to yield inner value: (*v).push(2)
            v.push(4);
            println!("sync_thread thr1: {:?}", v);
        } // mutex unlocks at the end of this scope(Drop Trait for MutexGuard )

        // Do Non critical work which need not be under critical section
    });
    handles.push(handle1);

    // ******************* Thread 2 *******************
    let wrapped_arc_mutex_clone2 = Arc::clone(&wrapped_arc_mutex);
    let handle2 = thread::spawn(move || {
        let mut v = wrapped_arc_mutex_clone2.lock().unwrap();
        v.push(3);
        v.push(5);
        println!("sync_thread thr2: {:?}", v);
    });

    // Join the threads
    handles.push(handle2);
    for handle in handles {
        handle.join()?;
    }
    Ok(())
}

pub fn scoped_thread() -> std::result::Result<i32, String> {
    let m_vec: Mutex<Vec<i32>> = Mutex::new(vec![0, 1]);

    /// The threads are joined before the scoped closure goes out of scope,
    /// eliminating the concern of vec going out of scope earlier than desired
    ///   pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    ///   where
    ///       F: FnOnce() -> T + Send + 'scope,
    ///       T: Send + 'scope,
    thread::scope(|s| {
        s.spawn(|| {
            let mut v: MutexGuard<Vec<i32>> = m_vec.lock().unwrap();
            v.push(2);
            v.push(4);
        });
        s.spawn(|| {
            let mut v = m_vec.lock().unwrap();
            v.push(3);
            v.push(5);
        });
        s.spawn(|| {
            let mut v = m_vec.lock().unwrap();
            v.push(8);
            v.push(9);
        });
    });
    // Collecting the sum of the vec elements into res
    let res: i32 = {
        let vector = m_vec.lock().unwrap();
        println!("scoped_thread vector: {:?}", vector);
        vector.iter().sum()
    };
    // Err("Dammit!".to_string())
    // Err(8)
    Ok(res)
}

////////////////////////////////////////////////////////
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}

// Message from Go Community :)
// Do not communicate by sharing memory; instead, share memory by communicating.
// Crossbeam: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel/benchmarks

pub fn msg_thread() -> thread::Result<()> {
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();

    // MPSC: Multi Producer Single Consumer Channel
    // Two parts: Transmitter/Sender(tx) and Receiver(rx)
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel::<u32>();

    /////////////////////////////////////////////////////
    // Thread 1 - Producer
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for msg in (2..=8).filter(|n| *n % 2 == 1) {
            tx1.send(msg).unwrap();
            println!("Tx1: {msg}");
        }
    });
    handles.push(handle1);

    /////////////////////////////////////////////////////
    // Thread 2 - Producer
    let handle2 = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(2));
        for msg in (20..=28).filter(|n| *n % 2 == 1) {
            tx.send(msg).unwrap();
            println!("Tx3: {msg}");
        }
    });
    handles.push(handle2);

    /////////////////////////////////////////////////////
    // Main thread - Consumer
    let mut v = vec![0, 1];
    for received in rx {
        v.push(received);
    }
    println!("Message thread - Vec: {:?}", v);

    // Join the threads
    for handle in handles {
        handle.join()?;
    }
    Ok(())
}
