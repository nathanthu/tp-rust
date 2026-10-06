#![allow(clippy::ptr_arg)]

// TODO : Corrigez les erreurs de compilation sans rien changer d'autre que
// l'ajout ou la suppression de références (le caractère `&`).

// Ne devrait PAS prendre la possession (ownership) de `data`
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Devrait prendre la possession (ownership) de `data`
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust, c'est génial !".to_string();

    get_char(data);

    string_uppercase(&data);
}
