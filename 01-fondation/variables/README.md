## Partie 1 : immuabilité

1- Déclare une variable immuable ville avec la valeur "Cotonou".
2- Essaie volontairement de lui assigner "Porto-Novo".
3- Lance cargo build et lis le message d’erreur en entier. Note ce que le compilateur te suggère.
4- Corrige le code.

## Partie 2 : mut

1- Déclare un compteur tentatives à 0.
2- Incrémente-le trois fois, puis affiche sa valeur.
3- Essaie ensuite de lui assigner une chaîne de caractères, observe l’erreur, puis retire cette ligne.

## Partie 3 : shadowing

1- Déclare saisie avec la valeur " 42 " (avec des espaces).
2- Par shadowing, redéclare saisie pour retirer les espaces (cherche la méthode dans la doc de str).
3- Par shadowing encore, convertis saisie en nombre (cherche parse, et note que tu auras un Result, un indice pour le niveau 3 !).
4- Affiche saisie + 1.

## Partie 4 : portée
Écris un bloc { } à l’intérieur duquel tu masques une variable, et vérifie ce qu’elle vaut avant, dans, et après le bloc.

Pour valider, réponds à ces questions dans tes propres mots :

- Pourquoi la partie 3 ne pourrait-elle pas se faire avec mut seul ?
- Dans quel cas préfères-tu mut plutôt que le shadowing ?