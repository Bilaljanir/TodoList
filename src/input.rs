use std::io;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture de l'entrée utilisateur");
    input.trim().to_string()
}