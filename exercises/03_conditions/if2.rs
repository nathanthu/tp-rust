// TODO : Corrigez l'erreur de compilation dans cette fonction.
fn picky_eater(food: &str) -> &str {
    if food == "fraise" {
        "Miam !"
    } else {
        1
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

// TODO : Lisez les tests pour comprendre le comportement attendu.
// Faites passer tous les tests sans les modifier.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Appeler `picky_eater` avec l'argument "fraise" doit renvoyer "Miam !".
        assert_eq!(picky_eater("fraise"), "Miam !");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("patate"), "Bon, je peux manger ça.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("brocoli"), "Non merci !");
        assert_eq!(picky_eater("bonbons"), "Non merci !");
        assert_eq!(picky_eater("n'importe quoi"), "Non merci !");
    }
}
