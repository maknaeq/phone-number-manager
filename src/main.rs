use phone_number_manager::{build_trie, read_contacts, to_plantuml};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        eprintln!("Usage: phone-number-manager <fichier.json>");
        process::exit(1);
    }

    let path = &args[0];

    let contacts = match read_contacts(path) {
        Ok(contacts) => contacts,
        Err(e) => {
            eprintln!("Erreur en lisant {} : {}", path, e);
            process::exit(1);
        }
    };

    let trie = build_trie(&contacts);
    let text = to_plantuml(&trie);
    let name = Path::new(path).file_stem().unwrap().to_str().unwrap();
    let output = format!("graph/{}.puml", name);

    match std::fs::write(&output, text) {
        Ok(()) => println!("graph écrit dans le fichier {}", output),
        Err(e) => {
            eprintln!("une erreur est survenue à l'écriture : {}", e);
            process::exit(1);
        }
    }
}
