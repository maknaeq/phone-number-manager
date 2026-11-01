#![forbid(unsafe_code)]

use serde::Deserialize;
use std::error::Error;
// Les erreurs du programme. (Quentin)

// Le trie qui range les numéros. (Kaan)

// Lit le fichier JSON des contacts. (Quentin)
#[derive(Deserialize, Debug)]
pub struct Contact {
    nb: String,
    name: String,
}

pub fn read_contacts(path: &str) -> Result<Vec<Contact>, Box<dyn Error>> {
    let json = std::fs::read_to_string(path)?;
    let contacts = serde_json::from_str(&json)?;
    Ok(contacts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_all_contacts_from_valid_file() {
        let contacts = read_contacts("data/04_common_parts.json").unwrap();
        assert_eq!(contacts.len(), 5);
        assert_eq!(contacts[0].nb, "0412578440");
        assert_eq!(contacts[0].name, "Alice");
    }

    #[test]
    fn reads_empty_list() {
        let contacts = read_contacts("data/08_empty.json").unwrap();
        assert!(contacts.is_empty());
    }

    #[test]
    fn missing_file_returns_error() {
        assert!(read_contacts("data/does_not_exist.json").is_err());
    }

    #[test]
    fn malformed_json_returns_error() {
        assert!(read_contacts("data/invalid_format.json").is_err());
    }
}

// Écrit le trie dans un fichier PlantUML. (Timothée)
