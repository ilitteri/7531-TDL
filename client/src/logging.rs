use std::io;

pub struct AccountCredentials {
    pub dni: String,
    pub password: String
}

impl AccountCredentials {
    pub fn new(dni: &str ,password: &str) -> Self {
        AccountCredentials {
            dni: dni.to_string(),
            password: password.to_string()
        }
    }
}

pub fn ask_for_log() -> AccountCredentials{
    let mut dni = String::new();
    let mut password = String::new();

    println!("Ingresa tu DNI:");
    io::stdin()
        .read_line(&mut dni)
        .expect("Failed to read line");

    println!("Ingresa tu contraseña:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");


    let client_account = AccountCredentials::new(
        dni.trim(),
        password.trim(),
    );
    println!("Tus datos son estos:");
    println!("DNI: {}", client_account.dni);
    println!("Contraseña: {}", client_account.password);

    return client_account;
}