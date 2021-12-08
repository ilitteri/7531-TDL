use std::io;
use crate::client_account::ClientAccount;

const YES:&str = "y";
const NO:&str = "n";
const RISK_AGE:i32 = 60;
const HIGH_PRIORITY:i32=1;
const MIDDLE_PRIORITY:i32=2;
const LOW_PRIORITY:i32=3;

const LEN_DNI:i32 = 8;

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

    
    let dni = ask_for_dni();
    let priority = ask_for_priority();

    let client_account = ClientAccount::new(
        name.trim(),
        lastname.trim(),
        email.trim(),
        password.trim(),
        birth_date.trim(),
        dni.trim(),
        priority.trim(),
    );

    println!("Tus datos son estos:");
    println!("Nombre: {}", client_account.name.clone().unwrap());
    println!("Apellido: {}", client_account.lastname.clone().unwrap());
    println!("Email: {}", client_account.email.clone().unwrap());
    println!("Contraseña: {}", client_account.password.clone().unwrap());
    println!("Fecha de nacimiento: {}", client_account.birth_date.clone().unwrap());
    println!("DNI: {}", client_account.dni.clone().unwrap());
    println!("Prioridad de turno: {}", client_account.priority.clone().unwrap());

    return client_account;
}


pub fn ask_for_priority() -> String {
    let mut option_str = String::new();

    println!("Ingrese su edad: ");
    io::stdin()
        .read_line(&mut option_str)
        .expect("Failed to read line");
    let age_string = option_str.trim().to_string();
    let age = age_string.parse::<i32>().expect("Error con el parse");

    let mut answer = String::new();
    while &answer != YES && &answer != NO{
        answer = String::new();
        println!("¿Tiene o tuvo patologias asociadas a un mayor riesgo de enfermarse gravemente por COVID-19?");
        println!("Ingrese 'Y' si la respuesta es afirmativa o 'N' si la respuesta es no");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer= answer.trim().to_string();
    }
    
    return set_priority(age, set_pathologies(answer));
}

fn set_pathologies (has_pathologies: String)-> bool{
    let mut pathologies=false;
    if &has_pathologies == YES{
        pathologies=true;
    }
    return pathologies;
}


fn set_priority(age: i32, has_pathologies:bool) -> String{
    let mut priority = LOW_PRIORITY;
    if age >=RISK_AGE && has_pathologies {
        priority = HIGH_PRIORITY;
    }
    else if (age >=RISK_AGE && !has_pathologies) || (age < RISK_AGE && has_pathologies) {
        priority = MIDDLE_PRIORITY;
    }

    return priority.to_string();
}

fn ask_for_dni() -> String{
    let mut answer = String::new();
    println!("Ingresa tu DNI:");
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    answer= answer.trim().to_string();
    while answer.len() != LEN_DNI{
        answer = String::new();
        println!("Ingrese un DNI valido por favor: ");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            answer= answer.trim().to_string();
        }
    return answer;
}