# Chaînes de caractères

Rust possède deux types de chaînes :

- `&str` : une « tranche » de chaîne (string slice), empruntée et non modifiable ;
  c'est le type des littéraux comme `"bonjour"` ;
- `String` : une chaîne possédée (owned), stockée sur le tas, qui peut grandir.

C'est une application directe de l'ownership : `String` possède ses données,
`&str` les emprunte.

## Pour aller plus loin

- [Les chaînes de caractères (Rust Book)](https://doc.rust-lang.org/book/ch08-02-strings.html)
