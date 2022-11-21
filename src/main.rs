#![allow(dead_code, unused_variables, unused_doc_comments)]
mod basic_thread;
mod closure;
mod copysemantics;

fn main() {
    // copysemantics::copysemantics();
    // closure::closure_iterator();
    // basic_thread::basic_thread1();
    // basic_thread::basic_thread2();
    basic_thread::basic_thread3();
    // basic_thread::basic_thread4();
    // basic_thread::basic_thread5();

    // basic_thread::sync_thread().unwrap();

    // let result = basic_thread::scoped_thread();
    // match result {
    //     Ok(res) => println!("scoped_thread Result: {res}"),
    //     Err(err) => println!("{:?}", err),
    // }
    // basic_thread::msg_thread().unwrap();
}
