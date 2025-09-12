

fn main(){
    let arr = [10, 20, 30, 40];
    let maybe_value = arr.get(5); //Prende un elemento che non essite
    match maybe_value {
        Some(v) => println!("Trovatp {}",v),
        None => println!("Non trovato nessun valore "),//non so scrivere in italiano quindi scrivo un commento per non voler correggere

    }
}

//use std::io;
/*
fn with_result() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<i32>(){
        Ok(num) => println!("Numero valido{}", num),
        Err(_) => println!("Non hai inserito un numero valido"),
    }
}*/
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
