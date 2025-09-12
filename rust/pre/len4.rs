

fn main() {
    let mut s = String::from("ciao");
    borrow(&s);
    change(&mut s);

    print!("Finale {}", s);
}

fn borrow(s: &String){
    println!("Ho letto: {}", s);
}

fn change(s: &mut String){
    s.push_str(" mondo");
}

/*
fn ownership(){
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{:?}", s2);

    //Passaggio di proprieta
}
*/
