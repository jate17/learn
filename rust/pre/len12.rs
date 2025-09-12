use std::fs;

/*Metadati file*/
fn main() -> std::io::Result<()>{
    let metadata = fs::metadata("test.txt")?;
    println!("Dimensione {:?} byte", metadata.len() );
    println!("Is file {:?} ", metadata.is_file());
    println!("Is dir {:?} ", metadata.is_dir());
    Ok(())

}

/*

Gestion cartelle
use std::fs;

fn main() -> std::io::Result<()>{
    fs::create_dir("len12_test")?;
    fs::write("len12_test/file.txt", "Len12")?;

    for entry in fs::read_dir("len12_test")? {
        let entry = entry?;
        println!("{:?}", entry.path());
    }

    Ok(())
}

*/





/*
Lettura  con buf reader piu efficiente

use std::io::{self, BufRead, BufReader};
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    for linea in reader.lines(){
        println!("{}",linea?);
    }

    Ok(())
}
*/


/*

LEttura

use std::io::{self, BufRead, BufReader};
use std::io::{self, Read};

fn main() -> io::Result<()>{
    let mut file = File::open("test.txt")?;
    let mut contenuto = String::new();
    file.read_to_string(&mut contenuto)?;
    println!("{}", contenuto );
    Ok(())
}
*/

/*

Scrittura
use std::io::{self, BufRead, BufReader};
use std::io::Write;

fn main() -> std::io::Result<()>{
    let mut file = File::create("test.txt")?;
    writeln!(file, "Ciao, rust!")?;
    Ok(())
}
*/
