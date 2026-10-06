fn animal_habitat(animal: &str) -> &str {
    // TODO : Corrigez l'erreur de compilation dans l'instruction ci-dessous.
    let identifier = if animal == "crabe" {
        1
    } else if animal == "taupe" {
        2.0
    } else if animal == "serpent" {
        3
    } else {
        "Inconnu"
    };

    // Ne modifiez pas l'expression ci-dessous !
    if identifier == 1 {
        "Plage"
    } else if identifier == 2 {
        "Terrier"
    } else if identifier == 3 {
        "Désert"
    } else {
        "Inconnu"
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

// Ne modifiez pas les tests !
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mole_lives_in_burrow() {
        assert_eq!(animal_habitat("taupe"), "Terrier")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("serpent"), "Désert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crabe"), "Plage")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaure"), "Inconnu")
    }
}
