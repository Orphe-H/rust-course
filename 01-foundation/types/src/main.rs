fn main() {
    // part 1: bool
    let fichier_existe: bool = true;
    let valeur_vide: bool = false;

    println!("10 > 5 : {}", 10 > 5);
    println!("3 == 4 : {}", 3 == 4);

    if fichier_existe {
        println!("Fichier existe.");
    } else {
        println!("Fichier n'existe pas.");
    }

    // part 2 : usize
    let cle = "DATABASE_URL";
    let clen_len: usize = cle.len();
    let nombre: i32 = 7;
    // let somme = clen_len + nombre;
    /* l'erreur dit : error[E0308]: mismatched types
      --> src/main.rs:21:28
       |
    21 |     let somme = clen_len + nombre;
       |                            ^^^^^^ expected `usize`, found `i32`

    error[E0277]: cannot add `i32` to `usize`
       --> src/main.rs:21:26
        |
     21 |     let somme = clen_len + nombre;
        |                          ^ no implementation for `usize + i32`
        |
        = help: the trait `Add<i32>` is not implemented for `usize`
    help: the following other types implement trait `Add<Rhs>` */
    let somme = clen_len + nombre as usize;
    println!("somme = {}", somme);

    // part 3 : &str et string
    let url = "postgres://localhost";
    let mut converted_url = url.to_string();
    converted_url.push_str(":5432");

    println!("converted str = {}", converted_url);

    // url.push_str(":5432");
    /* l'erreur dit : error[E0599]: no method named `push_str` found for reference `&str` in the current scope
      --> src/main.rs:44:9
       |
    44 |     str.push_str(":5432");
       |         ^^^^^^^^ method not found in `&str` */

    let url_to_string: &str = &converted_url;
    println!("url_to_string = {}", url_to_string);

    // part 4 : minis-cas
    let ligne = "DATABASE_URL=";

    println!("la longueur de ligne est {}", ligne.len());

    let ligne_contient_symbole: bool = ligne.contains("=");

    if ligne_contient_symbole {
        println!("Ligne contient =");
    } else {
        println!("Ligne ne contient pas =");
    }

    let ligne_est_vide = ligne.is_empty();
    if ligne_est_vide {
        println!("Ligne est vide");
    } else {
        println!("Ligne n'est pas vide");
    }
}

// Q1 : "Bobby" est un &str parce qu'il est emprunter et stocker de façon optimale parce qu'on ne va pas modifier sa valeur.
// Q2 : .len() retourne un usize plutôt qu'un i32 parce que le usize peut contenir des valeurs très grandes et forcément
// non signés car elle contient la longueur de strucutre qui peuvent être vide ou contenir un peu ou énormément d'éléments.
// Q3 : j'ai besoin d'un String plutôt qu'un &str quand je dois modifier la valeur de la variable.
