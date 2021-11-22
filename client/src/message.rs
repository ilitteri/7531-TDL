use std::io::Write;
use std::net::TcpStream;
use crate::client_account::ClientAccount;

pub enum Message {
    Log,
    Form,
    Disconnect,
    Unknown
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

pub fn send_log(stream: &mut TcpStream) {
    let buffer = [Message::Log.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

pub fn send_disconnect(stream: &mut TcpStream) {
    let buffer = [Message::Disconnect.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

fn calculate_lenght(form :&ClientAccount) -> u8 {
    let mut lenght:u8 = 0;
    lenght += (form.get_dni().unwrap().len() + 2) as u8;
    lenght += (form.get_lastname().unwrap().len() + 2) as u8;
    lenght += (form.get_birth_date().unwrap().len() + 2) as u8;
    lenght += (form.get_password().unwrap().len() + 2) as u8;
    lenght += (form.get_email().unwrap().len() + 2) as u8;
    lenght += (form.get_name().unwrap().len() + 2) as u8;
    return lenght;
}

pub fn send_register(stream: &mut TcpStream, form :&ClientAccount) {
    let lenght = calculate_lenght(&form);
    println!("El tamaño de form es ->{}",lenght);
    let buffer = [Message::Form.into(), lenght];
    stream.write_all(&buffer).unwrap();
    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());

    buffer_envio.push((form.get_dni().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_dni().unwrap().len() as u8);
    let dni = form.get_dni().unwrap();
    let dni_bytes = dni.as_bytes();
    for i in 0..dni_bytes.len(){
        buffer_envio.push(dni_bytes[i]);
    }

    buffer_envio.push((form.get_password().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_password().unwrap().len() as u8);
    let password = form.get_password().unwrap();
    let password_bytes = password.as_bytes();
    for i in 0..password_bytes.len(){
        buffer_envio.push(password_bytes[i]);
    }

    buffer_envio.push((form.get_lastname().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_lastname().unwrap().len() as u8);
    let lastname = form.get_lastname().unwrap();
    let lastname_bytes = lastname.as_bytes();
    for i in 0..lastname_bytes.len(){
        buffer_envio.push(lastname_bytes[i]);
    }

    buffer_envio.push((form.get_name().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_name().unwrap().len() as u8);
    let name = form.get_name().unwrap();
    let name_bytes = name.as_bytes();
    for i in 0..name_bytes.len(){
        buffer_envio.push(name_bytes[i]);
    }

    buffer_envio.push((form.get_birth_date().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_birth_date().unwrap().len() as u8);
    let birth_date = form.get_birth_date().unwrap();
    let birth_bytes = birth_date.as_bytes();
    for i in 0..birth_bytes.len(){
        buffer_envio.push(birth_bytes[i]);
    }

    buffer_envio.push((form.get_email().unwrap().len() >> 8) as u8);
    buffer_envio.push(form.get_email().unwrap().len() as u8);
    let email = form.get_email().unwrap();
    let email_bytes = email.as_bytes();
    for i in 0..email_bytes.len(){
        buffer_envio.push(email_bytes[i]);
    }
    stream.write(&buffer_envio).unwrap();
}

