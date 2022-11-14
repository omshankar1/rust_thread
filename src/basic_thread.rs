#![allow(dead_code, unused_variables, unused_doc_comments)]
use std::thread;

////////////////////////////////////////////////////////
/// pub type Result<T> = crate::result::Result<T, Box<dyn Any + Send + 'static>>;
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}

////////////////////////////////////////////////////////
/// Demonstrates: Basic threading, Life times
pub fn basic_thread() -> thread::Result<Vec<i32>> {
    /// Starting the vec 'v' with [0, 1]
    let mut v = vec![0, 1];

    /// Not captured by reference but by moving ownership
    /// The vector 'v' is moved
    let handle1 = thread::spawn(|| {
        v.push(1);
        v.push(2);
        println!("Vec: {:?}", v);
        v
    });

    // v.push(3); // Borrow of moved value: v

    // let handle2 = thread::spawn(|| {
    //     // Closure may outlive v
    //     v.push(4);
    //     v.push(5);
    //     println!("Vec: {:?}", v);
    // });
    // handle2.join();

    let result_vec = handle1.join().unwrap();
    Ok(result_vec)
}

/*
pub fn basic_thread1() -> thread::Result<()> {
    // let amp_str = "Hello World";
    let v = vec![1, 2, 3, 4];

    let handle1 = thread::spawn(|| {
        println!("String: {:?}", v);
    });
    Ok(())
}
*/

////////////////////////////////////////////////////////
/// Demonstrates: Closure
pub fn iterator_demo() {
    /// Closure
    let v = (0..=10).collect::<Vec<_>>(); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    let iter_v1 = v
        .iter()
        .map(|n| n * n)
        .filter(|n| *n > 0)
        .zip((100..=110).collect::<Vec<i32>>());
    let v1 = iter_v1.collect::<Vec<_>>();
    println!("Iterator result: {:?}", v1);
}

pub fn fnptr() {
    use std::f64::consts::PI;
    fn area(r: i32) -> f64 {
        (r * r) as f64 * PI
    }
    /// Function Pointer
    let area_fnptr = area;
    println!("area fnptr: {}", area_fnptr(1));

    let v = (0..=10).collect::<Vec<_>>(); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    let area = v.iter().map(|r| area_fnptr);
}

pub fn fnclosure() {
    use std::f64::consts::PI;
    //////////////////////////////////////////////////////////
    /// Area using Closure with env capture 'PI'

    /// Fn Trait
    let capture_height = 2; // Passed in as a capture
    let area_closure1 = |r: i32| (r * r * capture_height) as f64 * PI;
    println!("area closure: {}", area_closure1(1));
    assert_fn(area_closure1);

    let v = (0..=10).collect::<Vec<_>>(); // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    let area = v.iter().map(|r| area_closure1);

    //////////////////////////////////////////////////////////
    /// FnMut Trait
    let mut capture_height = 2; // Passed in as a capture
    let mut area_closure2 = |r: i32| {
        capture_height = capture_height + 1;
        (r * r * capture_height) as f64 * PI
    };
    println!("area closure: {}", area_closure2(1));
    assert_fnmut(area_closure2);

    //////////////////////////////////////////////////////////
    /// FnOnce Trait
    let capture_height = 2; // Passed in as a capture
    let capture_height = 2; // Passed in as a capture
    let area_closure3 = move || {
        let capture_height = capture_height + 1;
        let r = 9;
        (r * r * capture_height) as f64 * PI
    };
    // Closure to rust thread needs to be FnOnce
    thread::spawn(area_closure3);
}

pub fn assert_fn<F>(f: F)
where
    F: Fn(i32) -> f64,
{
}
pub fn assert_fnmut<F>(f: F)
where
    F: FnMut(i32) -> f64,
{
}
