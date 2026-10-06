# Ownership (possession) et emprunt

Chaque valeur en Rust a un unique propriétaire (owner). Quand on passe une valeur à une
fonction, sa possession est déplacée (« move ») et la variable d'origine n'est plus
utilisable. Pour éviter cela, on peut cloner la valeur ou l'emprunter avec une référence
(`&` en lecture, `&mut` en écriture).

Règle d'or de l'emprunt : à un instant donné, on peut avoir **soit** plusieurs références
`&`, **soit** une seule référence `&mut`.

Ces exercices sont adaptés du [tutoriel Rust](https://pnkfelix.github.io/rust-examples-icfp2014/)
de [pnkfelix](https://github.com/pnkfelix). Merci Felix !

## Pour aller plus loin

- [Qu'est-ce que l'ownership ?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Références et emprunt](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
