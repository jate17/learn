use std::fs::File;
use std::io::{self, Read};


fn main(){
    //panic!("Test"); // Casi rari o di test


    /* Alternativa sicura */
    let f = File::open("config.txt");

    match f {
            Ok(_file) => println!("File aperto"),
            Err(e) => println!("Errore: {}", e)
    }


    match read_file() {
        Ok(txt) => println!("Contenuto:\n{}", txt),
        Err(e)  => eprintln!("Errore: {}", e),
    }
}


fn read_file() -> io::Result<String> {
    let mut f = File::open("config.txt")?;      // se Err -> ritorna subito Err
    let mut contenuto = String::new();
    f.read_to_string(&mut contenuto)?;
    Ok(contenuto)
}
