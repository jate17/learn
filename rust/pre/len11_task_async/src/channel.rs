use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messaggi = vec!["ciao", "da", "un", "thread"];
        for m in messaggi {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for ricevuto in rx {
        println!("Ricevuto: {}", ricevuto);
    }
}
