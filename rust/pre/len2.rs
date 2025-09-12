
fn main() {
    let x: i32 = 42;
    let pi: f64 = 3.14;
    let happy: bool = true;
    let letter: char = 'r';

    let tuple = (10,11,12);
    let array = [1,1,1,1];
}

fn greet(name: &str){
    println!("Ciao, {}", name);
}

fn add(a: i32, b: i32) -> i32{
    a + b // Interessante valore ritornato se non mettiamo il return
}

fn controllo_flusso() {
    let num = 7;

    if num & 2 == 0 {
        println!("Pari");
    }else{
        println!("Dispari");
    }

    for i in 0..10{
        println!("I = {}", i);
    }

    let mut count = 0 ;
    while count < 3 {
        println!("Count = {}", count);
        count += 1;
    }
}
