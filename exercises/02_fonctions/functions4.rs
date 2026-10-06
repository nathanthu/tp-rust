// Ce magasin fait des soldes : si le prix est pair, vous avez 10 Rustbucks de
// réduction, s'il est impair, 3 Rustbucks de réduction.
// Ne vous préoccupez pas du corps des fonctions, seules leurs signatures nous
// intéressent pour l'instant.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO : Corrigez la signature de la fonction.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Votre prix soldé est de {}", sale_price(original_price));
}
