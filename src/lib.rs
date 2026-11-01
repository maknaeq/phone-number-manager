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

// Écrit le trie dans un fichier PlantUML. (Timothée)
