use std::collections::HashMap;

fn main(){

    /*

    Stack VS Heap VS Static


    Stack -> Veloce, dimensione fissa, complie-time(Bipolarismo del compilatore che decide) -> Fine dello scopes
    Heap -> Più lento -> Dimensioen variabiel -> Finche c'è owner c'è speranza
    Static -> Fisso -> Tutto il programma

    */

    // Stack -> Dimensione fissa -> a fine programma scopo viene liberata
    let x: i32 = 30;

    //Stack + heap -> Stack contenitore, heap hello
    let s: String = String::from("Hello");

    //Owenership cambia
    let s2 = s;

    //Duplicato su heap
    let s3 = s2.clone();

    /*Questo in Stack*/
    let mut v = Vec::new();

    /*Questo in heap*/
    v.push(10);
    v.push(20);
    v.push(30);

    /*Questo in stack*/
    let mut mappa = HashMap::new();


    /*Questo viene messa in static  */
    mappa.insert("Utente1","Password");
    mappa.insert("Utente2","Password12");

    // Questa viene messa in heap
    mappa.insert(String::from("Utente3"), String::from("Password"));
    mappa.insert(String::from("Utente4"), String::from("Password12"));
}

}
