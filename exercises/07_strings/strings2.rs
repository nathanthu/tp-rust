// TODO : Corrigez l'erreur de compilation dans la fonction `main`, sans
// modifier cette fonction-ci.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "vert" || attempt == "bleu" || attempt == "rouge"
}

fn main() {
    let word = String::from("vert"); // Ne modifiez pas cette ligne.

    if is_a_color_word(word) {
        println!("Je connais cette couleur !");
    } else {
        println!("Je ne connais pas cette couleur.");
    }
}
