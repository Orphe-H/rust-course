# Cours : Les types : `String`, `&str`, `bool`, `usize`

**Rappel : Rust est typé statiquement**

Chaque valeur a un type connu à la compilation. Rust le déduit souvent tout seul (`let x = 5;`), mais tu peux l’écrire explicitement avec :.

```rust
let age: u32 = 25;
```

1. **`bool`**

Deux valeurs possibles : `true` et `false`.

```rust
let est_vide: bool = true;
let est_grand = 10 > 5; // le résultat d'une comparaison est un bool
```

C’est le type des conditions : `if` exige un `bool`. Contrairement à d’autres langages, Rust ne convertit pas un nombre ou une chaîne en booléen.

```rust
let n = 1;
if n { }       // ❌ n n'est pas un bool
if n == 1 { }  // ✅
```

2. **`usize`**

Un entier **non signé** (jamais négatif), dont la taille dépend de ta machine (64 bits sur un système 64 bits). C’est le type utilisé pour :

- les **longueurs** (`.len()` retourne un `usize`)
- les **indices** dans les collections

```rust
let texte = "DATABASE_URL";
let longueur: usize = texte.len();
```

Rust a plusieurs types d’entiers : `i32` (signé, le défaut), `u32`, `u64`, `i64`, `usize`, etc. Le `u` signifie unsigned (pas de négatif), le `i` signifie signed.

**Piège fréquent** : Rust ne convertit jamais automatiquement entre types numériques.

```rust
let a: usize = 5;
let b: i32 = 3;
let c = a + b; // ❌ on ne mélange pas usize et i32
```

Il faut convertir explicitement, par exemple `a + b as usize`. Ça paraît pénible au début, mais ça évite toute une catégorie de bugs.

Et comme `usize` ne peut pas être négatif, `0 - 1` sur un usize provoque un _panic_ (ou une erreur à la compilation si Rust le détecte).

3. **`&st`r et `String` : le point le plus important**

Les deux représentent du texte, mais ce ne sont pas la même chose.

**`&str` : une vue sur du texte**

Un `&str` (on dit « _string slice_ ») est une **référence vers du texte qui existe déjà quelque part**. Tu ne le possèdes pas, tu le regardes.

```rust
let nom = "Bobby"; // les littéraux de texte sont de type &str
```

Il est **non modifiable** et de taille fixe.

**`String` : du texte que tu possèdes**

Un String est une chaîne **détenue** par ta variable, stockée en mémoire dynamique. Elle peut grandir et être modifiée.

```rust
let mut nom = String::from("Bobby");
nom.push_str(" Orphé");
println!("{}", nom); // Bobby Orphé
```

**Comparaison**

|                        | &str                          | String                |
| :--------------------- | :---------------------------- | :-------------------- |
| Qui possède le texte ? | Quelqu’un d’autre Ta variable |
| Modifiable ?           | Non                           | Oui (avec mut)        |
| Taille                 | Fixe                          | Peut grandir          |
| Exemple                | "Bobby"                       | String::from("Bobby") |

Une analogie : `&str` est comme **consulter un livre à la bibliothèque** (tu le lis, tu ne peux pas l’annoter). `String` est comme **posséder ton propre cahier** (tu peux écrire dedans, le rallonger).

**Passer de l’un à l’autre**

```rust
// &str vers String
let a: String = "Bobby".to_string();
let b: String = String::from("Bobby");

// String vers &str : on emprunte avec &
let c: &str = &a;
```

**À retenir**

- `bool` : `true` / `false`, indispensable pour les conditions.
- `usize` : entier positif pour les longueurs et indices, pas de mélange automatique avec les autres types numériques.
- `&str` : texte **emprunté**, non modifiable.
- `String` : texte **possédé**, modifiable.

**À consulter**

- The **Rust Programming Language**, chapitre 3.2 : **Data Types** (pour `bool` et les entiers).
- Chapitre 8.2 : **Storing UTF-8 Encoded Text with Strings**.
- La doc standard de `String` et de `str` (cherche les méthodes `len`, `is_empty`, `trim`, `to_uppercase`, `push_str`, `contains`).

Lis ces ressources **après** avoir tenté l’exercice.

# Exercice

## Partie 1 : bool

1. Déclare deux variables bool : fichier_existe (vraie) et valeur_vide (fausse), avec leur type écrit explicitement.
2. Affiche le résultat de la comparaison 10 > 5 et celui de 3 == 4.
3. Écris un if qui affiche un message différent selon la valeur de fichier_existe.

## Partie 2 : usize

1. Déclare une variable cle contenant "DATABASE_URL".
2. Récupère sa longueur dans une variable de type usize écrit explicitement, et affiche-la.
3. Essaie d’additionner cette longueur à un i32. Lis l’erreur, puis corrige en convertissant.

## Partie 3 : &str et String

1. Crée un &str contenant "postgres://localhost".
2. Convertis-le en String, rends-le mut, puis ajoute ":5432" à la fin. Affiche le résultat.
3. Essaie d’appeler push_str directement sur le &str d’origine. Observe l’erreur.
4. Depuis ton String, obtiens un &str (avec &) et affiche-le.

## Partie 4 : mini-cas

Déclare une ligne let ligne = "DATABASE_URL=";, puis sans chercher à parser pour l’instant :

1. Affiche sa longueur.
2. Vérifie avec une méthode de str si elle contient le caractère = et stocke le résultat dans un bool.
3. Vérifie si la ligne est vide avec is_empty().

Pour valider, réponds dans tes propres mots :

- Pourquoi "Bobby" est-il un &str et pas un String ?
- Pourquoi .len() retourne-t-il un usize plutôt qu’un i32 ?
- Dans quel cas as-tu besoin d’un String plutôt que d’un &str ?
