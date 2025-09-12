use std::io;

fn main(){
    //pow();
    dispa();
}

fn dispa(){
    let mut num = String::new();
    println!("Inserisci un numero:");
    io::stdin().read_line(&mut num).unwrap();

    match num.trim().parse::<i32>(){
        Ok(n) => {
            let msg = if n % 2 == 0 {"Pari"} else {"Dispari"};
            println!("{}", msg);
        },
        Err(_) => println!("PRoblemI"),
    }
}

/*
fn pow(){

    let mut num = String::new();
    println!("Inserisci un numero:");
    io::stdin().read_line(&mut num).unwrap();

    match num.trim().parse::<i32>(){
        Ok(n) => println!("Quadrato: {}", n*n),
        Err(_) => println!("Valore sbagliato"),
    }
}
*/
