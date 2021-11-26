use std::io;
use crate::client_account::ClientAccount;

const YES:char = 'Y';
const NO:char = 'N';
const RISK_AGE:i32 = 60;
const HIGH_PRIORITY:i32=1;
const MIDDLE_PRIORITY:i32=2;
const LOW_PRIORITY:i32=3;

pub fn ask_for_form() -> ClientAccount {
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

    println!("Ingresa tu DNI:");
    io::stdin()
        .read_line(&mut dni)
        .expect("Failed to read line");

    let mut priority = ask_for_priority();

    let client_account = ClientAccount::new(
        name.trim(),
        lastname.trim(),
        email.trim(),
        password.trim(),
        birth_date.trim(),
        dni.trim(),
    );

    println!("Tus datos son estos:");
    println!("Nombre: {}", client_account.name.clone().unwrap());
    println!("Apellido: {}", client_account.lastname.clone().unwrap());
    println!("Email: {}", client_account.email.clone().unwrap());
    println!("Contraseña: {}", client_account.name.clone().unwrap());
    println!("Fecha de nacimiento: {}", client_account.name.clone().unwrap());
    println!("DNI: {}", client_account.name.clone().unwrap());

    return client_account;
}


pub fn ask_for_priority() -> String {
    let mut option_str = String::new();
    let mut has_pathologies = 'A';

    println!("Ingrese su edad: ");
    io::stdin()
        .read_line(&mut option_str)
        .expect("Failed to read line");
    let age_string = option_str.trim().to_string();
    let age = age_string.parse::<i32>().expect("Error con el parse");

    while has_pathologies != YES || has_pathologies != NO {
        println!("¿Tiene o tuvo patologias asociadas a un mayor riesgo de enfermarse gravemente por COVID-19?");
        println!("Ingrese 'Y' si la respuesta es afirmativa o 'N' si la respuesta es no");
        io::stdin()
            .read_line(&mut option_str)
            .expect("Failed to read line");
        let pathologies_string = option_str.trim().to_string();
        has_pathologies = pathologies_string.parse::<char>().expect("Error con el parse");
    }
    
    return set_priority(age, has_pathologies);
}


fn set_priority(age: i32, has_pathologies:char) -> String{
    let mut priority = LOW_PRIORITY;
    if age >=RISK_AGE && has_pathologies==YES{
        priority = HIGH_PRIORITY;
    }
    else if (age >=RISK_AGE && has_pathologies==NO) || (age < RISK_AGE && has_pathologies==YES) {
        priority = MIDDLE_PRIORITY;
    }

    return priority.to_string();
}