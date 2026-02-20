use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b 
}

fn main() {
    let scanner = io::stdin();

    println!("Opération : Addition (A + B)");

    println!("Entrez la valeur de A :");
    let mut a = String::new();
    scanner.read_line(&mut a).expect("Erreur lecture A");

    println!("Entrez la valeur de B :");
    let mut b = String::new();
    scanner.read_line(&mut b).expect("Erreur lecture B");

    {
        let a: i32 = a.trim().parse().expect("Erreur de conversion A");
        let b: i32 = b.trim().parse().expect("Erreur de conversion B");
        println!("Le résultat est {}", add(a, b));
    }

    println!("Le type de A : {} et celui de B : {} ne change pas",a.trim(), b.trim());
}