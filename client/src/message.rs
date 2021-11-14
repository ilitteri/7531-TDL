use std::io::Write;
use std::net::TcpStream;

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