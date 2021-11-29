use std::io::{Read, Write};
use std::net::TcpStream;
use crate::client_account::ClientAccount;
use crate::logged_menu;
use crate::logging::AccountCredentials;

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

pub fn send_disconnect(stream: &mut TcpStream) {
    let buffer = [Message::Disconnect.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

pub fn send_shutdown(stream: &mut TcpStream) {
    let buffer = [Message::Shutdown.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

fn calculate_lenght(form :&ClientAccount) -> u8 {
    let mut lenght:u8 = 0;
    lenght += (form.get_dni().unwrap().len() + 1) as u8;
    lenght += (form.get_lastname().unwrap().len() + 1) as u8;
    lenght += (form.get_birth_date().unwrap().len() + 1) as u8;
    lenght += (form.get_password().unwrap().len() + 1) as u8;
    lenght += (form.get_email().unwrap().len() + 1) as u8;
    lenght += (form.get_name().unwrap().len() + 1) as u8;
    lenght += (form.get_priority().unwrap().len() + 1) as u8;
    
    return lenght;
}

pub fn send_register(stream: &mut TcpStream, form :&ClientAccount) {
    let lenght = calculate_lenght(&form);
    println!("El tamaño del form es ->{}",lenght);
    let buffer = [Message::Form.into(), lenght];
    stream.write_all(&buffer).unwrap();
    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());

    buffer_envio.push(form.get_dni().unwrap().len() as u8);
    let dni = form.get_dni().unwrap();
    let dni_bytes = dni.as_bytes();
    for i in 0..dni_bytes.len(){
        buffer_envio.push(dni_bytes[i]);
    }

    buffer_envio.push(form.get_password().unwrap().len() as u8);
    let password = form.get_password().unwrap();
    let password_bytes = password.as_bytes();
    for i in 0..password_bytes.len(){
        buffer_envio.push(password_bytes[i]);
    }

    buffer_envio.push(form.get_lastname().unwrap().len() as u8);
    let lastname = form.get_lastname().unwrap();
    let lastname_bytes = lastname.as_bytes();
    for i in 0..lastname_bytes.len(){
        buffer_envio.push(lastname_bytes[i]);
    }

    buffer_envio.push(form.get_name().unwrap().len() as u8);
    let name = form.get_name().unwrap();
    let name_bytes = name.as_bytes();
    for i in 0..name_bytes.len(){
        buffer_envio.push(name_bytes[i]);
    }

    buffer_envio.push(form.get_birth_date().unwrap().len() as u8);
    let birth_date = form.get_birth_date().unwrap();
    let birth_bytes = birth_date.as_bytes();
    for i in 0..birth_bytes.len(){
        buffer_envio.push(birth_bytes[i]);
    }

    buffer_envio.push(form.get_email().unwrap().len() as u8);
    let email = form.get_email().unwrap();
    let email_bytes = email.as_bytes();
    for i in 0..email_bytes.len(){
        buffer_envio.push(email_bytes[i]);
    }

    buffer_envio.push(form.get_priority().unwrap().len() as u8);
    let priority = form.get_priority().unwrap();
    let priority_bytes = priority.as_bytes();
    for i in 0..priority_bytes.len(){
        buffer_envio.push(priority_bytes[i]);
    }

    stream.write(&buffer_envio).unwrap();
}

fn calculate_lenght_log(log :&AccountCredentials) -> u8 {
    let mut lenght:u8 = 0;
    lenght += (log.get_dni().unwrap().len() + 1) as u8;
    lenght += (log.get_password().unwrap().len() + 1) as u8;
    return lenght;
}

pub fn send_log(stream: &mut TcpStream, log :&AccountCredentials) {
    let lenght = calculate_lenght_log(&log);
    println!("El tamaño del log es ->{}",lenght);
    let buffer = [Message::Log.into(), lenght];
    stream.write_all(&buffer).unwrap();
    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());

    buffer_envio.push(log.get_dni().unwrap().len() as u8);
    let dni = log.get_dni().unwrap();
    let dni_bytes = dni.as_bytes();
    for i in 0..dni_bytes.len(){
        buffer_envio.push(dni_bytes[i]);
    }

    buffer_envio.push(log.get_password().unwrap().len() as u8);
    let password = log.get_password().unwrap();
    let password_bytes = password.as_bytes();
    for i in 0..password_bytes.len(){
        buffer_envio.push(password_bytes[i]);
    }
    stream.write(&buffer_envio).unwrap();
}

pub fn read_response_from_server(stream: &mut TcpStream) {
    let mut num_buffer = [0u8; 2]; //Recibimos 2 bytes
    stream.read_exact(&mut num_buffer);
    match  Message::from(num_buffer[0]) {
        Message::Nice => {
            println!("Se loggeo correctamente!");
            //Lanzar otro menu donde pueda ver cuando le toco el turno, su info y desloggearse.
            logged_menu(stream);
        }
        Message::Error => {
            println!("Hubo un error al trata de autentificarse!");
        }
        _ => {
            println!("Nose que me contesto el server!");
        }
    }
}