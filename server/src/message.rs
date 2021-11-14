use std::io::Read;
use std::net::{Shutdown, TcpStream};
use std::process::exit;

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

pub fn read_message(stream: &mut TcpStream, buffer: u8, message_type: Message) -> Result<(), std::io::Error> {
    let mut buffer_packet: Vec<u8> = vec![0; buffer as usize];
    stream.read_exact(&mut buffer_packet);
    match message_type {
        Message::Log => {
            println!("Recibi un intento de log!");
        }
        Message::Form => {
            println!("Recibi un formulario!");
        }
        Message::Unknown => {
            println!("Nose que paso!");
        }
        Message::Disconnect => {
            println!("El cliente se desconecto :,C!");
            println!("Cierro el stream y el thread!");
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        _ => {
            println!("Nose que paso!");
        }
    }
    Ok(())
}
