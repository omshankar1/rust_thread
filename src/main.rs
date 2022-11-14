#![allow(dead_code)]
mod basic_thread;
mod msg_thread;
mod scoped_thread;
mod sync_thread;

fn main() {
    // basic_thread::iterator_demo();

    let result = basic_thread::basic_thread();
    match result {
        Ok(vec) => println!("Handling Result: {:?}", vec),
        Err(err) => println!("{:?}", err),
    }

    // sync_thread::sync_thread().unwrap();

    // scoped_thread::sync_thread().unwrap();
    // msg_thread::msg_thread();
}
