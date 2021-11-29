use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientAccount {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
    pub dni: String,
    pub priority: String
}

fn main() {
    let path = "ejemplo.json";
    write_json(path);
    let client_account = read_json(path);
}

// TODO: ESCRIBIR JSON
pub fn write_json(path: &str) {
    let name = "Ivan".to_string();
    let lastname = "Litteri".to_string();
    let email = "ilitteri@fi.uba.ar".to_string();
    let password = "abcd".to_string();
    let birth_date = "13/04/2000".to_string();
    let dni = "42646324".to_string();
    let priority = "2".to_string();
    
    let client_account: ClientAccount = ClientAccount { 
        name: name,
        lastname: lastname,
        email: email,
        password: password,
        birth_date: birth_date,
        dni: dni,
        priority: priority
     }; 
    
    println!("{}", serde_json::to_string(&client_account).unwrap());
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