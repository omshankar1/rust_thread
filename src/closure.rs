#![allow(dead_code, unused_variables, unused_doc_comments, unused_imports)]
use std::sync::{Arc, Mutex};
use std::thread;

////////////////////////////////////////////////////////
pub fn basic_thread1() {
    let v = "Hello";
    let v = vec![1, 2, 3];
    let v = &v[..];

    let handle1 = thread::spawn(move || {
        // println!("String: {:?}", v);
    });
    handle1.join().unwrap();
}

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

pub fn fnclosures() {
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
    let area_closure3 = move || {
        let r = 5;
        let area = (r * r * capture_height) as f64 * PI;
    };
    // Closure to rust thread needs to be FnOnce
    thread::spawn(area_closure3);

    //////////////////////////////////////////////////////////
    // FnOnce Trait
    let str = "Hello".to_string();
    let closure_fn_once = || drop(str);
    assert_fnonce_noargs(closure_fn_once);
}

pub fn assert_fn<F>(f: F)
where
    F: Fn(i32) -> f64,
{
    println!("{}", f(1));
}

pub fn assert_fnmut<F>(mut f: F)
where
    F: FnMut(i32) -> f64,
{
    println!("{}", f(1));
}

pub fn assert_fnonce<F>(f: F)
where
    F: FnOnce(i32) -> f64,
{
    println!("{}", f(1));
}

pub fn assert_fnonce_noargs<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
