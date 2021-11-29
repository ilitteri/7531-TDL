use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;

const ERROR: u8 = 1;

pub enum Message {
    Log,
    Form,
    Disconnect,
    Unknown,
    Nice,
    Error,
    Shutdown
}

impl From<u8> for Message {
    fn from(code :u8) -> Message {
        match code & 0xF0 {
            0x10 => Message::Log,
            0x20 => Message::Form,
            0x30 => Message::Disconnect,
            0x40 => Message::Nice,
            0x50 => Message::Error,
            0x97 => Message::Shutdown,
            _=> Message::Unknown
        }
    }
}

impl From<Message> for u8 {
    fn from(code: Message) -> u8 {
        match code {
            Message::Log => 0x10,
            Message::Form => 0x20,
            Message::Disconnect =>  0x30,
            Message::Nice => 0x40,
            Message::Error => 0x50,
            Message::Shutdown => 0x97,
            _ => 0x99
        }
    }
}

fn bytes2string(bytes: &[u8]) -> Result<String, u8> {
    match std::str::from_utf8(bytes) {
        Ok(str) => Ok(str.to_owned()),
        Err(_) => Err(ERROR)
    }
}

fn leer_contenido_formulario(buffer_packet: Vec<u8>) -> Result<u8, u8> {
    let mut index = 0 as usize;
    let mut dni : Option<String> = None;
    let dni_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    dni = Some(bytes2string(&buffer_packet[index..(index + dni_size)])?);
    index += dni_size;

    println!("El Dni es -> {}", dni.unwrap());

    let mut password : Option<String> = None;
    let password_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    password = Some(bytes2string(&buffer_packet[index..(index + password_size)])?);
    index += password_size;

    println!("El password es -> {}", password.unwrap());

    let mut lastname : Option<String> = None;
    let lastname_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    lastname = Some(bytes2string(&buffer_packet[index..(index + lastname_size)])?);
    index += lastname_size;

    println!("El lastname es -> {}", lastname.unwrap());

    let mut name : Option<String> = None;
    let name_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    name = Some(bytes2string(&buffer_packet[index..(index + name_size)])?);
    index += name_size;

    println!("El name es -> {}", name.unwrap());

    let mut birth_date : Option<String> = None;
    let birth_date_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    birth_date = Some(bytes2string(&buffer_packet[index..(index + birth_date_size)])?);
    index += birth_date_size;

    println!("El birth_date es -> {}", birth_date.unwrap());

    let mut email : Option<String> = None;
    let email_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    email = Some(bytes2string(&buffer_packet[index..(index + email_size)])?);
    index += email_size;

    println!("El email es -> {}", email.unwrap());

    let mut priority : Option<String> = None;
    let priority_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    priority = Some(bytes2string(&buffer_packet[index..(index + priority_size)])?);
    index += priority_size;

    println!("La prioridad es -> {}", priority.unwrap());

    Ok(1)
}

fn leer_contenido_log(buffer_packet: Vec<u8>) -> Result<u8, u8> {
    let mut index = 0 as usize;
    let mut dni : Option<String> = None;
    let dni_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    dni = Some(bytes2string(&buffer_packet[index..(index + dni_size)])?);
    index += dni_size;

    println!("El Dni es -> {}", dni.unwrap());

    let mut password : Option<String> = None;
    let password_size: usize = buffer_packet[(index) as usize] as usize;
    index += 1 as usize;
    password = Some(bytes2string(&buffer_packet[index..(index + password_size)])?);
    index += password_size;

    println!("El password es -> {}", password.unwrap());

    Ok(1)
}

fn send_nice_log_message(stream: &mut TcpStream) {
    let buffer = [Message::Nice.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

fn send_error_log_message(stream: &mut TcpStream) {
    let buffer = [Message::Error.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

pub fn read_message(stream: &mut TcpStream, size: u8, message_type: Message) -> Result<(), std::io::Error> {
    let mut buffer_packet: Vec<u8> = vec![0; size as usize];
    let _aux = stream.read_exact(&mut buffer_packet); //Manejar
    match message_type {
        Message::Log => {
            println!("Recibi un intento de log!");
            let _aux1 = leer_contenido_log(buffer_packet); // Manejar
            // Aca cuando se tenga la base de clientes se analiza es un usuario correcto o no y se avisa al usuario
            send_nice_log_message(stream);
            println!("Envie que fue exitoso el log!");
            //Con condicionales segun corresponda
            /*send_error_log_message(&stream);
             */
        }
        Message::Form => {
            println!("Recibi un formulario!");
            let _aux2 = leer_contenido_formulario(buffer_packet); // Manejar
            // Aca cuando se tenga la base de clientes se analiza es un usuario correcto o no y se avisa al usuario
            send_nice_log_message(stream);
            println!("Envie que fue exitoso el form!");
            //Con condicionales segun corresponda
            /*send_error_log_message(&stream);
             */
        }
        Message::Unknown => {
            println!("Nose que paso!");
        }
        Message::Disconnect => {
            println!("El cliente se desconecto :,C!");
            println!("Cierro el stream y el thread!");
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        Message::Shutdown => {
            println!("Se procede a apagar el servidor!");
            //Me guardo todas las historias medicas q tenga del archivo de donde la lei y salgo
            exit(1);
        }
        _ => {
            println!("Unknown message!")
        }
    }
    Ok(())
}
