use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<i32>(){
        Ok(num) => println!("Numero valido{}", num),
        Err(_) => println!("Non hai inserito un numero valido"),
    }
}
/*
fn basic_mode() {
    let mut input = String::new();
    println!("Inserisci un numero: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Errore di lettura");

    println!("Hai scritto {}", input);
}
*/
