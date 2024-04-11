use std::io;
use tsar_client::Client;

const APP_ID: &str = "58816206-b24c-41d4-a594-8500746a78ee";
const PUBLIC_KEY: &str = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAELlyGTmNEv3AarudyshJUUA9ig1pOfSl5qWX8g/hkPiieeKlWvv9o4IZmWI4cCrcR0fteVEcUhBvu5GAr/ITBqA==";

fn main() {
    let api = Client::new(APP_ID, PUBLIC_KEY);

    match api.authenticate_user() {
        Ok(data) => println!("Success: {:?}", data), // Auth Success
        Err(err) => println!("\x1b[31m[AUTH ERROR] {:?}\x1b[0m: {}", err, err), // Auth Failed
    }

    println!("Press enter to close...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
