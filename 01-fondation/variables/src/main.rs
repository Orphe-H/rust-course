fn main() {
    // part 1 : immutabilité
    let ville = "Cotonou";
    // ville = "Porto-Novo";
    let ville2 = "Porto-Novo";

    // part 2 : mut
    let mut tentatives = 0;
    tentatives += 1;
    tentatives += 1;
    tentatives += 1;
    // tentatives = "4";
    println!("tentatives = {}", tentatives);

    // part 3 : shadowing
    let saisie = " 42 ";
    let saisie = str::trim(saisie);
    let saisie: u32 = saisie.parse().expect("Ce n'est pas un nombre.");
    println!("saisies = {}", saisie + 1);

    // part 4 : portée
    let nombre = "un";
    println!("nombre = {}", nombre);
    {
        let nombre = 2;
        println!("nombre = {}", nombre);
    }
    println!("nombre = {}", nombre);
}

// Q1 : parce qu'on veut changer le type de saisie en passant d'un string à un entier.
// Q2 : dans le cas où je veux garder le même type tout en modifier le contenu d'une variable et surtout 
// parce que le shadowing ne survit pas à une bouble