
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("Thread 1 dice ciao").unwrap();
    });

    thread::spawn(move || {
        tx1.send("Thread 2 dice ciao").unwrap();
    });

    for msg in rx {
        println!("Ricevuto: {}", msg);
    }
}
