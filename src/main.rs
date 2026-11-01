use phone_number_manager::read_contacts;

fn main() {
    let contacts = read_contacts("./data/contacts.json");
    match contacts {
        Ok(contacts) => {
            println!("{:#?}", contacts);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
