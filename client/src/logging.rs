use std::io;
use crate::form::ask_for_dni;

pub struct AccountCredentials {
    pub dni: Option<String>,
    pub password: Option<String>
}

impl AccountCredentials {
    pub fn new(dni: &str ,password: &str) -> Self {
        AccountCredentials {
            dni: Some(dni.to_string()),
            password: Some(password.to_string())
        }
    }
    pub fn get_dni(&self) -> Option<String> {
        return self.dni.clone();
    }

    pub fn get_password(&self) -> Option<String> {
        return self.password.clone();
    }
}

pub fn ask_for_log() -> AccountCredentials{
    let mut _dni = String::new();
    let mut password = String::new();

    let mut _dni = ask_for_dni();

    println!("Ingresa tu contraseña:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");


    let client_account = AccountCredentials::new(
        _dni.trim(),
        password.trim(),
    );
    println!("Tus datos son estos:");
    println!("DNI: {}", client_account.dni.clone().unwrap());
    println!("Contraseña: {}", client_account.password.clone().unwrap());

    return client_account;
}
