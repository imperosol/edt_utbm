mod requests;
mod parser;
mod generator;

use std::io::{stdin, stdout, Write};
use rpassword;


fn ask_credentials() -> (String, String) {
    let mut username = String::new();
    print!("Identifiant : ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();
    let password = rpassword::prompt_password("Mot de passe : ").unwrap();
    (username, password)
}

fn main() {
    let client = requests::init_client();
    let (username, password) = ask_credentials();
    println!("\nConnexion à l'espace étudiant...");
    requests::login(&client, username, password);
    let res = match requests::get_timetable_page(&client) {
        Ok(res) => res,
        Err(_) => {
            eprintln!("Impossible d'accéder au dossier étudiant, \
            peut-être en raison d'identifiants incorrects");
            std::process::exit(1);
        }
    };

    let ues = parser::get_ues(res.text().unwrap().as_str());
    generator::get_html(ues);

    println!("\nLe fichier `edt.html` a été créé");
}
