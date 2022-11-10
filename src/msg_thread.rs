use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time;

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
    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(1));
        for msg in (20..=28).filter(|n| *n % 2 == 1) {
            tx2.send(msg).unwrap();
            println!("Tx2: {msg}");
        }
    });
    handles.push(handle2);

    /////////////////////////////////////////////////////
    // Thread 3 - Producer
    let handle3 = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(2));
        for msg in (20..=28).filter(|n| *n % 2 == 1) {
            tx.send(msg).unwrap();
            println!("Tx3: {msg}");
        }
    });
    handles.push(handle3);

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
