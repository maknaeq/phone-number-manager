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



// Écrit le trie dans un fichier PlantUML au format MindMap. (Timothé)

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

        let expected = "@startmindmap
                        * 1
                        ** 1
                        *** 2
                        **** Urgences
                        ** 5
                        *** SAMU
                        @endmindmap
                        ";
        assert_eq!(to_plantuml(&trie), expected);
    }
}
