use std::io::Read;
use std::net::{Shutdown, TcpStream};

const ERROR: u8 = 1;

pub enum Message {
    Log,
    Form,
    Unknown,
    Disconnect
}

impl From<u8> for Message {
    fn from(code :u8) -> Message {
        match code & 0xF0 {
            0x10 => Message::Log,
            0x20 => Message::Form,
            0x30 => Message::Disconnect,
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
    let mut indice = 0 as usize;
    let mut dni : Option<String> = None;
    let tamanio_dni: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    dni = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_dni)])?);
    indice += tamanio_dni;

    println!("El Dni es -> {}", dni.unwrap());

    let mut password : Option<String> = None;
    let tamanio_password: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    password = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_password)])?);
    indice += tamanio_password;

    println!("El password es -> {}", password.unwrap());

    let mut lastname : Option<String> = None;
    let tamanio_lastname: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    lastname = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_lastname)])?);
    indice += tamanio_lastname;

    println!("El lastname es -> {}", lastname.unwrap());

    let mut name : Option<String> = None;
    let tamanio_name: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    name = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_name)])?);
    indice += tamanio_name;

    println!("El name es -> {}", name.unwrap());

    let mut birth_date : Option<String> = None;
    let tamanio_birth_date: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    birth_date = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_birth_date)])?);
    indice += tamanio_birth_date;

    println!("El birth_date es -> {}", birth_date.unwrap());

    let mut email : Option<String> = None;
    let tamanio_email: usize = ((buffer_packet[indice] as usize) << 8) + buffer_packet[(indice+1) as usize] as usize;
    indice += 2 as usize;
    email = Some(bytes2string(&buffer_packet[indice..(indice+tamanio_email)])?);
    indice += tamanio_email;

    println!("El email es -> {}", email.unwrap());

    Ok(1)
}

pub fn read_message(stream: &mut TcpStream, size: u8, message_type: Message) -> Result<(), std::io::Error> {
    let mut buffer_packet: Vec<u8> = vec![0; size as usize];
    let _aux = stream.read_exact(&mut buffer_packet); //Manejar
    match message_type {
        Message::Log => {
            println!("Recibi un intento de log!");

        }
        Message::Form => {
            println!("Recibi un formulario!");
            let _aux2 = leer_contenido_formulario(buffer_packet); // Manejar
        }
        Message::Unknown => {
            println!("Nose que paso!");
        }
        Message::Disconnect => {
            println!("El cliente se desconecto :,C!");
            println!("Cierro el stream y el thread!");
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
    }
    Ok(())
}
