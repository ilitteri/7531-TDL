
use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]


// TODO: ESCRIBIR JSON
pub fn write_json(path: &str, form :&ClientAccount ) {
    let file = match File::open(&json_file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    if file.exists(){
        user_info = serde_json::to_string(&client_account).unwrap(); 
        file.write_all(user_info)
    }
}

pub fn read_json(path: &str) -> ClientAccount {
    let json_file_path = Path::new(path);
    let display = json_file_path.display();

    let file = match File::open(&json_file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let client_account: ClientAccount = serde_json::from_reader(file).unwrap();

    // TODO: Sacar
    println!("Tus datos son estos:");
    println!("Nombre: {}", client_account.name);
    println!("Apellido: {}", client_account.lastname);
    println!("Email: {}", client_account.email);
    println!("Contraseña: {}", client_account.password);
    println!("Fecha de nacimiento: {}", client_account.birth_date);
    println!("DNI: {}", client_account.dni);
    println!("Prioridad de turno: {}", client_account.priority);

    return client_account;
}