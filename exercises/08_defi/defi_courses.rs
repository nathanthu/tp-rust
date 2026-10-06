#![allow(clippy::ptr_arg)]

// Défi final : la liste de courses 🛒
//
// Ce défi récapitule tout ce que vous avez vu sur l'ownership :
// - emprunter en lecture (`&`)
// - emprunter en écriture (`&mut`)
// - transférer la possession (move)
// Il y a 4 TODO à résoudre. Ne modifiez que ce qui est demandé.

// TODO 1 : Complétez la fonction pour renvoyer le nombre d'articles de la liste.
// (Astuce : les vecteurs ont une méthode `len()`.)
fn count(list: &Vec<String>) -> usize {
    todo!()
}

// TODO 2 : Cette fonction doit ajouter `item` à la fin de la liste, sans en
// prendre la possession. Corrigez uniquement le type du paramètre `list`.
fn add_item(list: &Vec<String>, item: &str) {
    list.push(item.to_string());
}

// TODO 3 : Cette fonction prend la possession de la liste et la renvoie triée
// par ordre alphabétique. Un seul mot-clé à ajouter !
fn sorted(list: Vec<String>) -> Vec<String> {
    list.sort();
    list
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shopping_list() {
        let mut list = Vec::new();
        add_item(&mut list, "pommes");
        add_item(&mut list, "beurre");
        add_item(&mut list, "farine");
        assert_eq!(count(&list), 3);

        let mut list = sorted(list);
        assert_eq!(list, ["beurre", "farine", "pommes"]);

        // TODO 4 : Ce bloc ne compile pas. Corrigez-le uniquement en
        // réordonnant ses trois lignes (sans en ajouter, modifier ou supprimer).
        let first = &list[0];
        add_item(&mut list, "oeufs");
        assert_eq!(first, "beurre");

        assert_eq!(count(&list), 4);
    }
}
