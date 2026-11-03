use phone_number_manager::read_contacts;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        eprintln!("Usage: phone-number-manager <fichier.json>");
        process::exit(1);
    }

    let path = &args[0];

    match read_contacts(path) {
        Ok(contacts) => {
            println!("{:#?}", contacts);
        }
        Err(e) => {
            eprintln!("Erreur en lisant {} : {}", path, e);
            process::exit(1);
        }
    }
}
