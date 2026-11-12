# Cours : Variables en Rust : let, mut, shadowing

**1. let : déclarer une variable**

En Rust, on crée une variable avec `let` :

```rust
let age = 25;
let nom = "Bobby";
```

Point central : **par défaut, une variable est immuable**. Une fois qu’elle a reçu une valeur, on ne peut plus la modifier.

```rust
let age = 25;
age = 26; // ❌ Erreur de compilation
```

Le compilateur refuse et te dit que tu ne peux pas assigner deux fois à une variable immuable. Ce n’est pas une contrainte gratuite : en Rust, on part du principe qu’une valeur qui ne change pas est plus sûre et plus facile à raisonner. Si une valeur doit changer, tu dois le dire explicitement.

Rust déduit le type tout seul (ici `i32` pour `age` et `&str` pour `nom`), mais tu peux aussi l’écrire :

```rust
let age: u32 = 25;
```

**2. mut : rendre une variable modifiable**

Pour autoriser la modification, on ajoute `mut` (pour mutable) :

```rust
let mut compteur = 0;
compteur = 1;
compteur += 1;
println!("{}", compteur); // 2
```

Quand utiliser `mut` ? Seulement quand la valeur doit vraiment évoluer (compteur, accumulateur, structure qu’on remplit). Dans ton projet Env Doctor, tu en auras besoin pour une collection qu’on remplit ligne après ligne.

Une limite importante : `mut` permet de **changer la valeur, pas le type**.

```rust
let mut x = 5;
x = "cinq"; // ❌ x est un entier, pas une chaîne
```

**3. Shadowing : redéclarer avec le même nom**

Le shadowing consiste à déclarer une **nouvelle** variable avec le même nom, avec un nouveau `let`. La nouvelle variable « masque » l’ancienne.

```rust
let x = 5;
let x = x + 1; // nouvelle variable x, vaut 6
let x = x * 2; // nouvelle variable x, vaut 12
println!("{}", x); // 12
```

À première vue, ça ressemble à `mut`, mais la différence est réelle :

|                                    | `mut`             | Shadowing   |
| :--------------------------------- | :---------------- | :---------- |
| Syntaxe                            | `x = 6`           | `let x = 6` |
| Crée une nouvelle variable         | Non               | Oui         |
| Peut changer de type               | Non               | Oui         |
| La variable reste immuable ensuite | Non (reste `mut`) | Oui         |

Le changement de type est le cas d’usage le plus courant :

```rust
let espaces = " "; // &str
let espaces = espaces.len(); // usize : on réutilise le nom, mais le type change
```

Avec `mut`, ce serait impossible. Le shadowing est donc très pratique quand on transforme une valeur étape par étape (par exemple une ligne de texte qu’on nettoie, puis qu’on découpe).

**Attention à la portée** : le shadowing dans un bloc `{ }` ne dure que dans ce bloc.

```rust
let x = 1;
{
    let x = 99;
    println!("{}", x); // 99
}
println!("{}", x);     // 1 : l'ancien x est de retour
```

**À retenir**

- `let` : variable **immuable** par défaut.
- `let mut` : variable **modifiable**, mais le type reste fixe.
- Shadowing (`let` répété) : **nouvelle** variable, le type peut changer.

**À consulter**
Dans _The Rust Programming Language_ (« le Book »), chapitre 3.1 : **Variables and Mutability**. Lis-le après avoir tenté l’exercice, pas avant.

# Exercice

## Partie 1 : immuabilité

1. Déclare une variable immuable ville avec la valeur "Cotonou".
2. Essaie volontairement de lui assigner "Porto-Novo".
3. Lance cargo build et lis le message d’erreur en entier. Note ce que le compilateur te suggère.
4. Corrige le code.

## Partie 2 : mut

1. Déclare un compteur tentatives à 0.
2. Incrémente-le trois fois, puis affiche sa valeur.
3. Essaie ensuite de lui assigner une chaîne de caractères, observe l’erreur, puis retire cette ligne.

## Partie 3 : shadowing

1. Déclare saisie avec la valeur " 42 " (avec des espaces).
2. Par shadowing, redéclare saisie pour retirer les espaces (cherche la méthode dans la doc de str).
3. Par shadowing encore, convertis saisie en nombre (cherche parse, et note que tu auras un Result, un indice pour le niveau 3 !).
4. Affiche saisie + 1.

## Partie 4 : portée

Écris un bloc { } à l’intérieur duquel tu masques une variable, et vérifie ce qu’elle vaut avant, dans, et après le bloc.

**Pour valider**, réponds à ces questions dans tes propres mots :

- Pourquoi la partie 3 ne pourrait-elle pas se faire avec mut seul ?
- Dans quel cas préfères-tu mut plutôt que le shadowing ?
