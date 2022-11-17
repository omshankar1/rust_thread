#![allow(dead_code, unused_variables, unused_doc_comments)]
mod basic_thread;
mod closure;
mod msg_thread;

fn main() {
    // basic_thread::iterator_demo();
    // basic_thread::basic_thread1();
    // basic_thread::basic_thread2();
    // basic_thread::basic_thread3();
    // basic_thread::basic_thread4();
    // basic_thread::basic_thread5();

    basic_thread::sync_thread().unwrap();

    let result = basic_thread::scoped_thread();
    match result {
        Ok(res) => println!("Handling Result: {res}"),
        Err(err) => println!("{:?}", err),
    }
    // msg_thread::msg_thread();
}
