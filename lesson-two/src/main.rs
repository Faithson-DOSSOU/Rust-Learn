use std::io;

fn main(){
    println!("Devinez le nombre !");
    println!("Entrez un nombre compris entre 0 et 100");

    let mut chiffre = String::new();

    /*io::stdin()
        .read_line(&mut chiffre)
        .expect("Erreur lors de la lecture de la ligne");*/

    let scanner = io::stdin();
    scanner.read_line(&mut chiffre)
        .expect("Erreur lors de la lecture de la ligne");

    println!("Vous avez choisi le chiffre : {chiffre}");
    
}

/*
Note : 

Ce code est le résultat de l'apprentissage du chapitre 2 de la documentation officielle du langage Rust : 
"https://doc.rust-lang.org/book/"

*/