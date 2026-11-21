# Cours : Les fonctions : paramètres, retour, signatures

1. **Déclarer une fonction**

On utilise le mot-clé fn. Tu en connais déjà une : main.

```rust
fn dire_bonjour() {
	println!("Bonjour !");
}

fn main() {
	dire_bonjour();
}
```

Par convention, les noms de fonctions sont en snake_case (minuscules séparées par des _). Peu importe l’ordre dans le fichier : on peut appeler une fonction définie plus bas.

2. **Les paramètres**

Les paramètres se déclarent entre parenthèses, et leur type est obligatoire (Rust ne le déduit jamais dans une signature).

```rust
fn afficher_cle(nom: &str, longueur: usize) {
	println!("{} fait {} octets", nom, longueur);
}

fn main() {
	afficher_cle("DATABASE_URL", 12);
}
```

C’est ici que ce que tu viens d’apprendre sur &str et String devient concret : la signature dit précisément ce que la fonction attend.

```rust
fn lire(texte: &str) { } // reçoit une vue sur du texte
fn garder(texte: String) { } // reçoit un texte qu'elle possède désormais
```
Une règle utile : quand une fonction a seulement besoin de lire du texte, prends un &str. Elle accepte alors aussi bien un littéral qu’un String emprunté avec &.

3. **La valeur de retour**

Le type retourné se place après ->.

```rust
fn additionner(a: i32, b: i32) -> i32 {
	a + b
}
```

Remarque : il n’y a pas de point-virgule après a + b. C’est volontaire.

Expression ou instruction

C’est la subtilité principale du chapitre :

Une instruction fait quelque chose et ne produit pas de valeur (se termine par ;).
Une expression produit une valeur (pas de ;).

La dernière expression d’une fonction, sans point-virgule, est sa valeur de retour.

```rust
fn double(x: i32) -> i32 {
	x * 2 // ✅ retourne la valeur
}

fn double_casse(x: i32) -> i32 {
	x * 2; // ❌ le ; transforme l'expression en instruction
}
```
La seconde ne compile pas : la fonction promet un i32 mais ne retourne rien. L’erreur du compilateur te suggère d’ailleurs de retirer le ;. C’est une erreur très courante au début.

Tu peux aussi utiliser return pour sortir plus tôt, par exemple dans un if :

```rust
fn est_positif(n: i32) -> bool {
	if n < 0 {
		return false;
	}
	true
}
```

4. **Fonction sans valeur de retour**

Quand on ne met pas de ->, la fonction retourne en réalité (), le type « unité » (une valeur vide). C’est le cas de main et de dire_bonjour ci-dessus.

5. **La signature, c’est un contrat**

Lis une signature comme une phrase :

```rust
fn longueur_de_cle(ligne: &str) -> usize
```

« Je prends une vue sur du texte, et je te rends un entier positif. » On sait ce qu’elle fait sans lire son corps. En Rust, les signatures sont l’outil principal pour comprendre un code.

6. **Retourner plusieurs valeurs : le tuple**

Un tuple regroupe plusieurs valeurs, éventuellement de types différents.

```rust
fn analyser(ligne: &str) -> (usize, bool) {
	(ligne.len(), ligne.is_empty())
}

fn main() {
	let (longueur, est_vide) = analyser("ABC=1");
	println!("{} {}", longueur, est_vide);
}
```

Tu peux aussi accéder aux éléments par position : resultat.0, resultat.1.

**À retenir**
fn nom(param: Type) -> TypeRetour { ... }
Les types des paramètres et du retour sont toujours écrits.
La dernière expression sans ; est la valeur retournée.
&str en paramètre pour lire, String quand la fonction doit posséder.
Sans ->, la fonction retourne ().
À consulter
The Rust Programming Language, chapitre 3.3 : Functions.
Chapitre 3.2, section sur les tuples (dans Compound Types).

Lis-les après avoir tenté l’exercice.

# Exercice

## Partie 1 : fonction simple

Écris une fonction afficher_titre() sans paramètre ni retour, qui affiche "=== Rust course - Bobby Orphé ===". Appelle-la depuis main.

## Partie 2 : paramètres

Écris afficher_cle(nom: &str, longueur: usize) qui affiche le nom d’une clé et sa longueur. Appelle-la avec "DATABASE_URL", en passant sa vraie longueur obtenue avec .len().

## Partie 3 : valeur de retour

Écris contient_egal(ligne: &str) -> bool qui retourne true si la ligne contient =.
Écris est_commentaire(ligne: &str) -> bool qui retourne true si la ligne commence par # (cherche la méthode dans la doc de str).
Teste-les avec "DATABASE_URL=x", "# commentaire" et "".

## Partie 4 : l’erreur du point-virgule

Écris volontairement une fonction qui retourne un i32 avec un ; sur la dernière ligne. Lance cargo build, lis le message d’erreur en entier et note ce que le compilateur te suggère. Puis corrige.

## Partie 5 : return anticipé

Écris ligne_utile(ligne: &str) -> bool qui retourne false si la ligne est vide ou si c’est un commentaire, true sinon. Utilise au moins un return anticipé, et appelle tes fonctions précédentes plutôt que de réécrire leur logique.

## Partie 6 : tuple

Écris analyser(ligne: &str) -> (bool, usize) qui retourne si la ligne contient = et sa longueur. Récupère le résultat dans main de deux façons : par déstructuration (let (a, b) = ...) puis par accès positionnel (.0, .1).

**Pour valider**, réponds dans tes propres mots :

- Pourquoi la signature d’une fonction doit-elle toujours écrire les types de ses paramètres ?
- Quelle est la différence entre une instruction et une expression, et en quoi le ; change-t-il le comportement d’une fonction ?
- Pour contient_egal, pourquoi prends-tu un &str plutôt qu’un String ?
