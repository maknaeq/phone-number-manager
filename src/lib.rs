#![forbid(unsafe_code)]

use serde::Deserialize;
use std::error::Error;
use std::fmt;

// Les erreurs du programme. (Quentin)

// Erreur renvoyée quand un numéro du fichier JSON n'est pas un numéro valide.
#[derive(Debug, PartialEq)]
pub struct InvalidNumberError {
    pub nb: String,
    pub name: String,
}

impl fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "numéro invalide \"{}\" pour {} (le numéro doit contenir uniquement des chiffres)",
            self.nb, self.name
        )
    }
}

impl Error for InvalidNumberError {}

// Un numéro valide n'est pas vide et ne contient que des chiffres.
pub fn is_valid_number(nb: &str) -> bool {
    !nb.is_empty() && nb.chars().all(|c| c.is_ascii_digit())
}

// Le trie qui range les numéros. (Kaan)

#[derive(Debug, Default)]
pub struct Node {
    children: Vec<(char, Node)>,
    name: Option<String>,
}

impl Node {
    pub fn children(&self) -> Vec<(char, &Node)> {
        let mut result = Vec::new();
        for (digit, node) in &self.children {
            result.push((*digit, node));
        }
        result
    }

    pub fn name(&self) -> Option<&str> {
        match &self.name {
            Some(name) => Some(name),
            None => None,
        }
    }

    fn find_child(&self, digit: char) -> Option<&Node> {
        for (child_digit, node) in &self.children {
            if *child_digit == digit {
                return Some(node);
            }
        }
        None
    }

    fn find_or_create_child(&mut self, digit: char) -> &mut Node {
        let mut index = 0;
        while index < self.children.len() {
            if self.children[index].0 == digit {
                return &mut self.children[index].1;
            }
            index = index + 1;
        }

        self.children.push((digit, Node::default()));
        &mut self.children[index].1
    }
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: Node::default(),
            len: 0,
        }
    }

    pub fn insert(&mut self, number: &str, name: &str) -> Option<String> {
        let mut node = &mut self.root;
        for digit in number.chars() {
            node = node.find_or_create_child(digit);
        }

        let old_name = node.name.clone();
        node.name = Some(name.to_string());

        if old_name == None {
            self.len = self.len + 1;
        }
        old_name
    }

    pub fn get(&self, number: &str) -> Option<&str> {
        let mut node = &self.root;
        for digit in number.chars() {
            match node.find_child(digit) {
                Some(child) => node = child,
                None => return None,
            }
        }
        node.name()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn root(&self) -> &Node {
        &self.root
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
#[derive(Deserialize, Debug)]
pub struct Contact {
    nb: String,
    name: String,
}

pub fn read_contacts(path: &str) -> Result<Vec<Contact>, Box<dyn Error>> {
    let json = std::fs::read_to_string(path)?;
    let contacts: Vec<Contact> = serde_json::from_str(&json)?;
    for contact in &contacts {
        if !is_valid_number(&contact.nb) {
            return Err(Box::new(InvalidNumberError {
                nb: contact.nb.clone(),
                name: contact.name.clone(),
            }));
        }
    }
    Ok(contacts)
}

#[cfg(test)]
mod json_tests {
    use super::*;

    #[test]
    fn reads_all_contacts_from_valid_file() {
        let contacts = read_contacts("data/valid/common_prefix.json").unwrap();
        assert_eq!(contacts.len(), 5);
        assert_eq!(contacts[0].nb, "0412578440");
        assert_eq!(contacts[0].name, "Alice");
    }

    #[test]
    fn reads_empty_list() {
        let contacts = read_contacts("data/valid/empty.json").unwrap();
        assert!(contacts.is_empty());
    }

    #[test]
    fn missing_file_returns_error() {
        assert!(read_contacts("data/does_not_exist.json").is_err());
    }

    #[test]
    fn malformed_json_returns_error() {
        assert!(read_contacts("data/invalid/trailing_comma.json").is_err());
    }

    #[test]
    fn number_with_letters_returns_error() {
        let error = read_contacts("data/invalid/bad_numbers.json").unwrap_err();
        let error = error.downcast_ref::<InvalidNumberError>().unwrap();
        assert_eq!(error.nb, "04AB578440");
        assert_eq!(error.name, "Lettres");
    }

    #[test]
    fn missing_name_returns_error() {
        assert!(read_contacts("data/invalid/missing_field.json").is_err());
    }

    #[test]
    fn number_not_a_string_returns_error() {
        assert!(read_contacts("data/invalid/wrong_type.json").is_err());
    }

    #[test]
    fn object_instead_of_list_returns_error() {
        assert!(read_contacts("data/invalid/not_an_array.json").is_err());
    }

    #[test]
    fn valid_numbers() {
        assert!(is_valid_number("0412578440"));
        assert!(is_valid_number("15"));
    }

    #[test]
    fn invalid_numbers() {
        assert!(!is_valid_number(""));
        assert!(!is_valid_number("04AB578440"));
        assert!(!is_valid_number("+33612345678"));
        assert!(!is_valid_number("06 12 34 56 78"));
    }
}

// Écrit le trie dans un fichier PlantUML au format MindMap. (Timothé)

// On met chaque contact dans le trie.
pub fn build_trie(contacts: &Vec<Contact>) -> Trie {
    let mut trie = Trie::new();
    for contact in contacts {
        trie.insert(&contact.nb, &contact.name);
    }
    trie
}

pub fn to_plantuml(trie: &Trie) -> String {
    let mut text = String::from("@startmindmap\n");
    add_children(&trie.root, 1, &mut text);
    text.push_str("@endmindmap\n");
    text
}

// construit le children avec une étoile * par niveau. par exemple pour samu on a : "* 1", puis "** 5", puis "*** SAMU"
fn add_children(node: &Node, depth: usize, text: &mut String) {
    let stars = "*".repeat(depth);
    for (digit, child) in node.children() {
        text.push_str(&format!("{stars} {digit}\n"));
        if let Some(name) = child.name() {
            text.push_str(&format!("{stars}* {name}\n"));
        }
        add_children(child, depth + 1, text);
    }
}

#[cfg(test)]
mod plantuml_tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie = Trie::default();
        assert_eq!(to_plantuml(&trie), "@startmindmap\n@endmindmap\n");
    }

    #[test]
    fn test_urgences_and_samu() {
        // trie avec 112 -> Urgences et 15 -> SAMU
        let node_112 = Node { children: vec![], name: Some("Urgences".to_string()) };
        let node_11 = Node { children: vec![('2', node_112)], name: None };
        let node_15 = Node { children: vec![], name: Some("SAMU".to_string()) };
        let node_1 = Node { children: vec![('1', node_11), ('5', node_15)], name: None };
        let mut trie = Trie::default();
        trie.root.children.push(('1', node_1));

        let expected = [
            "@startmindmap",
            "* 1",
            "** 1",
            "*** 2",
            "**** Urgences",
            "** 5",
            "*** SAMU",
            "@endmindmap",
            "",
        ]
        .join("\n");
        assert_eq!(to_plantuml(&trie), expected);
    }

    #[test]
    fn test_number_inside_another() {
        // 112 est le début de 1123 donc le nom "Urgences" est au milieu de la branche
        let mut trie = Trie::new();
        trie.insert("112", "Urgences");
        trie.insert("1123", "Standard");

        let expected = [
            "@startmindmap",
            "* 1",
            "** 1",
            "*** 2",
            "**** Urgences",
            "**** 3",
            "***** Standard",
            "@endmindmap",
            "",
        ]
        .join("\n");
        assert_eq!(to_plantuml(&trie), expected);
    }

    #[test]
    fn test_different_roots_from_json() {
        // test avec des roots différent entre les numéros "standard" (les 04) et les numéros d'urgence qui commencent par 1
        let contacts = read_contacts("data/valid/common_prefix.json").unwrap();
        let trie = build_trie(&contacts);
        let text = to_plantuml(&trie);
        assert!(text.contains("\n* 0\n"));
        assert!(text.contains("\n* 1\n"));
    }
}
