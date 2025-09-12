use std::thread;
use std::sync::{Arc, Mutex};

fn main(){
    let counter = Arc::new(Mutex::new(0)); //Count protetto da mutex
    let mut handles = vec![];

    for _ in 0..5{
        let counter = Arc::clone(&counter); //Copia di riferimento
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;  // modifico il contatore
        });
        handles.push(handle);
    }


    for h in handles {
        h.join().unwrap();
    }

    println!("Ris: {}",*counter.lock().unwrap());
}


/*semplice thread
fn main() {
    let heandle = thread::spawn( || {
        println!("Ciao thread");
    });

    heandle.join().unwrap();
    println!("Ciao main");
}

*/
