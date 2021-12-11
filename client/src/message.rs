use std::io::{Read, Write};
use std::net::TcpStream;
use crate::client_account::ClientAccount;
use crate::logged_menu;
use crate::logging::AccountCredentials;

const ERROR: u8 = 1;

pub enum Message {
    Log,
    Form,
    Disconnect,
    Unknown,
    Nice,
    Error,
    Shutdown,
    Appointment,
    Delete
}

impl From<u8> for Message {
    fn from(code :u8) -> Message {
        match code & 0xF0 {
            0x00 => Message::Delete,
            0x10 => Message::Log,
            0x20 => Message::Form,
            0x30 => Message::Disconnect,
            0x40 => Message::Nice,
            0x50 => Message::Error,
            0x60 => Message::Shutdown,
            0x70 => Message::Appointment,
            _=> Message::Unknown
        }
    }
}

impl From<Message> for u8 {
    fn from(code: Message) -> u8 {
        match code {
            Message::Delete => 0x00,
            Message::Log => 0x10,
            Message::Form => 0x20,
            Message::Disconnect =>  0x30,
            Message::Nice => 0x40,
            Message::Error => 0x50,
            Message::Shutdown => 0x60,
            Message::Appointment => 0x70,
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

fn push_to_buffer(buffer: &mut Vec<u8>, data: String) {
    buffer.push(data.len() as u8);
    let data_bytes = data.as_bytes();
    for i in 0..data_bytes.len(){
        buffer.push(data_bytes[i]);
    }
}

pub fn send_register(stream: &mut TcpStream, form :&ClientAccount) {
    let lenght = calculate_lenght(&form);
    let buffer = [Message::Form.into(), lenght];
    stream.write_all(&buffer).unwrap();
    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());

    push_to_buffer(&mut buffer_envio, form.get_dni().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_password().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_lastname().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_name().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_email().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_birth_date().unwrap());
    push_to_buffer(&mut buffer_envio, form.get_priority().unwrap());

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
    let buffer = [Message::Log.into(), lenght];
    stream.write_all(&buffer).unwrap();
    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());

    push_to_buffer(&mut buffer_envio, log.get_dni().unwrap());
    push_to_buffer(&mut buffer_envio, log.get_password().unwrap());
    stream.write(&buffer_envio).unwrap();
}

pub fn read_response_from_server(stream: &mut TcpStream) {
    let mut num_buffer = [0u8; 2];
    let _aux = stream.read_exact(&mut num_buffer);
    match  Message::from(num_buffer[0]) {
        Message::Nice => {
            println!("\nSe inició sesión correctamente\n");
            logged_menu(stream);
        }
        Message::Error => {
            println!("\nHubo un error al iniciar sesión\n");
        }
        _ => {
            println!("\nNo se que me contesto el server!\n");
        }
    }
}

pub fn send_consult(stream: &mut TcpStream) {
    let buffer = [Message::Appointment.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

pub fn send_delete(stream: &mut TcpStream) {
    let buffer = [Message::Delete.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

fn bytes2string(bytes: &[u8]) -> Result<String, u8> {
    match std::str::from_utf8(bytes) {
        Ok(str) => Ok(str.to_owned()),
        Err(_) => Err(ERROR)
    }
}

pub fn read_date(stream: &mut TcpStream) -> Result<ClientAccount, u8>{
    let mut num_buffer = [0u8; 2];
    let _aux = stream.read_exact(&mut num_buffer);
    match  Message::from(num_buffer[0]) {
        Message::Appointment => {
            let mut buffer_packet: Vec<u8> = vec![0; num_buffer[1] as usize];
            let _aux = stream.read_exact(&mut buffer_packet);
            let mut _index = 0 as usize;
            let mut _dias: Option<String> = None;
            let days_size: usize = buffer_packet[(_index) as usize] as usize;
            _index += 1 as usize;
            _dias = Some(bytes2string(&buffer_packet[_index..(_index + days_size)])?);
            _index += days_size;
            println!("\nEl turno es en {} días\n", _dias.unwrap());
        }
        _ => {
            println!("\nNo se que me contesto el server!\n");
        }
    }
    Err(1)
}