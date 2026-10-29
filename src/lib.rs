#![forbid(unsafe_code)]

// Les erreurs du programme. (Quentin)



// Le trie qui range les numéros. (Kaan)

#[derive(Debug, Default)]
pub struct Node {
    children: Vec<(char, Node)>,
    name: Option<String>,
}

impl Node {
    pub fn children(&self) -> impl Iterator<Item = (char, &Node)> {
        self.children.iter().map(|(digit, node)| (*digit, node))
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, number: &str, name: &str) -> Option<String> {
        let _ = (number, name);
        todo!()
    }

    pub fn get(&self, number: &str) -> Option<&str> {
        let _ = number;
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn root(&self) -> &Node {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_example_trie() -> Trie {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        trie.insert("0412199803", "Bob");
        trie.insert("0468892011", "patate");
        trie.insert("112", "Urgences");
        trie.insert("15", "SAMU");
        trie
    }

    fn children_digits(node: &Node) -> Vec<char> {
        let mut digits = Vec::new();
        for (digit, _child) in node.children() {
            digits.push(digit);
        }
        digits
    }

    fn find_child(node: &Node, wanted_digit: char) -> &Node {
        for (digit, child) in node.children() {
            if digit == wanted_digit {
                return child;
            }
        }
        panic!("pas de child {}", wanted_digit);
    }

    fn count_nodes(node: &Node) -> usize {
        let mut total = 1;
        for (_digit, child) in node.children() {
            total = total + count_nodes(child);
        }
        total
    }

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert_eq!(trie.len(), 0);
        assert_eq!(trie.is_empty(), true);
        assert_eq!(trie.get("0412578440"), None);
    }

    #[test]
    fn test_insert_one_number() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        assert_eq!(trie.get("0412578440"), Some("Alice"));
        assert_eq!(trie.len(), 1);
        assert_eq!(trie.is_empty(), false);
    }

    #[test]
    fn test_get_all_numbers() {
        let trie = create_example_trie();
        assert_eq!(trie.len(), 5);
        assert_eq!(trie.get("0412578440"), Some("Alice"));
        assert_eq!(trie.get("0412199803"), Some("Bob"));
        assert_eq!(trie.get("0468892011"), Some("patate"));
        assert_eq!(trie.get("112"), Some("Urgences"));
        assert_eq!(trie.get("15"), Some("SAMU"));
    }

    #[test]
    fn test_unknown_number() {
        let trie = create_example_trie();
        assert_eq!(trie.get("0699999999"), None);
        assert_eq!(trie.get("113"), None);
    }

    #[test]
    fn test_start_of_number_not_found() {
        let trie = create_example_trie();
        assert_eq!(trie.get("0412"), None);
        assert_eq!(trie.get("1"), None);
    }

    #[test]
    fn test_number_too_long() {
        let trie = create_example_trie();
        assert_eq!(trie.get("1123"), None);
    }

    #[test]
    fn test_number_inside_another_number() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        trie.insert("1515", "Autre");
        assert_eq!(trie.get("15"), Some("SAMU"));
        assert_eq!(trie.get("1515"), Some("Autre"));
        assert_eq!(trie.len(), 2);
    }

    #[test]
    fn test_same_number_twice() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences");
        trie.insert("112", "Pompiers");
        assert_eq!(trie.get("112"), Some("Pompiers"));
        assert_eq!(trie.len(), 1);
    }

    #[test]
    fn test_common_start_is_shared() {
        let mut trie = Trie::new();
        trie.insert("0412578440", "Alice");
        let before = count_nodes(trie.root());
        trie.insert("0412199803", "Bob");
        let after = count_nodes(trie.root());
        assert_eq!(after - before, 6);
    }

    #[test]
    fn test_example_trie_shape() {
        let trie = create_example_trie();
        let root = trie.root();
        assert_eq!(children_digits(root), vec!['0', '1']);

        let node_0 = find_child(root, '0');
        let node_04 = find_child(node_0, '4');
        assert_eq!(children_digits(node_04), vec!['1', '6']);

        let node_1 = find_child(root, '1');
        assert_eq!(children_digits(node_1), vec!['1', '5']);
    }

    #[test]
    fn test_insertion_order() {
        let mut trie = Trie::new();
        trie.insert("9", "Neuf");
        trie.insert("1", "Un");
        trie.insert("5", "Cinq");
        assert_eq!(children_digits(trie.root()), vec!['9', '1', '5']);
    }

    #[test]
    fn test_name_on_last_node() {
        let trie = create_example_trie();
        let node_1 = find_child(trie.root(), '1');
        let node_11 = find_child(node_1, '1');
        let node_112 = find_child(node_11, '2');
        assert_eq!(node_11.name(), None);
        assert_eq!(node_112.name(), Some("Urgences"));
    }
}

// Lit le fichier JSON des contacts. (Quentin)



// Écrit le trie dans un fichier PlantUML. (Timothé)

// Tests plantuml.com pour visualiser l'arbre (SAMU et Urgences pour l'instant)

//@startuml
//object "racine" as root
//
//' commun alice bob (numéros qui commençant par 0)
//object "0" as n0
// object "4" as n04
// object "1" as n041
// object "2" as n0412
//
// ' alice : 0412578440
// object "5" as n04125
// object "7" as n041257
// object "8" as n0412578
// object "4" as n04125784
// object "4" as n041257844
// object "0" as n0412578440
// n0412578440 : alice
//
// ' bob : 0412199803
// object "1" as n04121
// object "9" as n041219
// object "9" as n0412199
// object "8" as n04121998
// object "0" as n041219980
// object "3" as n0412199803
// n0412199803 : bob
//
// ' patate : 0468892011
// object "6" as n046
// object "8" as n0468
// object "8" as n04688
// object "9" as n046889
// object "2" as n0468892
// object "0" as n04688920
// object "1" as n046889201
// object "1" as n0468892011
// n0468892011 : patate
//
// ' numéro qui commençant par 1
// object "1" as n1
// object "1" as n11
// object "2" as n112
// n112 : Urgences
// object "5" as n15
// n15 : SAMU
//
// root --> n0
// root --> n1
//
// n0 --> n04
// n04 --> n041
// n04 --> n046
// n041 --> n0412
//
// n0412 --> n04125
// n04125 --> n041257
// n041257 --> n0412578
// n0412578 --> n04125784
// n04125784 --> n041257844
// n041257844 --> n0412578440
//
// n0412 --> n04121
// n04121 --> n041219
// n041219 --> n0412199
// n0412199 --> n04121998
// n04121998 --> n041219980
// n041219980 --> n0412199803
//
// n046 --> n0468
// n0468 --> n04688
// n04688 --> n046889
// n046889 --> n0468892
// n0468892 --> n04688920
// n04688920 --> n046889201
// n046889201 --> n0468892011
//
// n1 --> n11
// n11 --> n112
// n1 --> n15
// @enduml

