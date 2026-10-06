fn bigger(a: i32, b: i32) -> i32 {
    // TODO : Complétez cette fonction pour qu'elle renvoie le plus grand nombre !
    // Si les deux nombres sont égaux, vous pouvez renvoyer n'importe lequel.
    // N'utilisez pas :
    // - d'autre appel de fonction
    // - de variable supplémentaire
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

// Ne vous occupez pas de ceci pour l'instant :)
// (Ce sont les tests qui vérifient votre solution.)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
