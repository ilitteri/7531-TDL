use std::io;

mod client_account;
use crate::client_account::ClientAccount;


fn main() {
    let mut name = String::new();
    let mut lastname = String::new();
    let mut email = String::new();
    let mut password = String::new();
    let mut birth_date = String::new();
    let mut dni = String::new();

    println!("Ingresá tu nombre:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Ingresa tu apellido:");
    io::stdin()
        .read_line(&mut lastname)
        .expect("Failed to read line");

    println!("Ingresa tu email:");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    println!("Ingresa tu contraseña:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    println!("Ingresa tu Fecha de Nacimiento:");
    io::stdin()
        .read_line(&mut birth_date)
        .expect("Failed to read line");
    
    println!("Ingresa:");
    io::stdin()
        .read_line(&mut dni)
        .expect("Failed to read line");
    
    let client_account = ClientAccount::new(
        name.trim(),
        lastname.trim(),
        email.trim(),
        password.trim(),
        birth_date.trim(),
        dni.trim(),
    );

    println!("Nombre: {}", client_account.name);
    println!("Apellido: {}", client_account.lastname);
    println!("Email: {}", client_account.email);
    println!("Contraseña: {}", client_account.name);
    println!("Fecha de nacimiento: {}", client_account.name);
    println!("DNI: {}", client_account.name);
}