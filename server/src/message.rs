use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex};

use crate::client_account::ClientAccount;
use crate::file_handling::{find_user, update_database, write_json};
use crate::appointments::get_appointment;
use crate::logging::check_credentials;
use crate::logging::AccountCredentials;
use crate::server::PATH;

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

fn bytes2string(bytes: &[u8]) -> Result<String, u8> {
    match std::str::from_utf8(bytes) {
        Ok(str) => Ok(str.to_owned()),
        Err(_) => Err(ERROR)
    }
}

fn read_forms(buffer_packet: Vec<u8>, mutex: &Arc<Mutex<Vec<ClientAccount>>>, dni_user: &mut String) -> Result<u8, u8> {
    let mut _index = 0 as usize;
    let mut _dni: Option<String> = None;
    let dni_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _dni = Some(bytes2string(&buffer_packet[_index..(_index + dni_size)])?);
    _index += dni_size;
    let aux_dni = _dni.clone().unwrap();
    *dni_user = aux_dni;

    let mut _password: Option<String> = None;
    let password_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _password = Some(bytes2string(&buffer_packet[_index..(_index + password_size)])?);
    _index += password_size;

    let mut _lastname: Option<String> = None;
    let lastname_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _lastname = Some(bytes2string(&buffer_packet[_index..(_index + lastname_size)])?);
    _index += lastname_size;

    let mut _name: Option<String> = None;
    let name_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _name = Some(bytes2string(&buffer_packet[_index..(_index + name_size)])?);
    _index += name_size;

    let mut _birth_date: Option<String> = None;
    let birth_date_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _birth_date = Some(bytes2string(&buffer_packet[_index..(_index + birth_date_size)])?);
    _index += birth_date_size;

    let mut _email: Option<String> = None;
    let email_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _email = Some(bytes2string(&buffer_packet[_index..(_index + email_size)])?);
    _index += email_size;

    let mut _priority: Option<String> = None;
    let priority_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _priority = Some(bytes2string(&buffer_packet[_index..(_index + priority_size)])?);
    _index += priority_size;

    let client_account = ClientAccount::new(&_name.unwrap(), &_lastname.unwrap(), &_email.unwrap(), &_password.unwrap(), &_birth_date.unwrap(), &_dni.unwrap(), &_priority.unwrap());

    mutex.lock().unwrap().push(client_account.clone());
    let _aux = write_json(PATH, client_account); // Seguro por si se cae el server

    Ok(1)
}

fn leer_contenido_log(buffer_packet: Vec<u8>, lock: &Arc<Mutex<Vec<ClientAccount>>>, algo: &mut String) -> Result<ClientAccount, u8> {
    let mut _index = 0 as usize;
    let mut _dni: Option<String> = None;
    let dni_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _dni = Some(bytes2string(&buffer_packet[_index..(_index + dni_size)])?);
    _index += dni_size;
    let aux = _dni.clone().unwrap();
    *algo = aux;

    let mut _password: Option<String> = None;
    let password_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _password = Some(bytes2string(&buffer_packet[_index..(_index + password_size)])?);
    _index += password_size;

    let account_credentials = AccountCredentials::new(&_dni.clone().unwrap(), &_password.unwrap());
    if let Ok(account)= find_user(lock, _dni.clone().unwrap()){
        return check_credentials(account_credentials, account);
    }
    Err(1)
}

fn send_nice_log_message(stream: &mut TcpStream) {
    let buffer = [Message::Nice.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

fn send_error_log_message(stream: &mut TcpStream) {
    let buffer = [Message::Error.into(), 0_u8];
    stream.write_all(&buffer).unwrap();
}

pub fn read_message(stream: &mut TcpStream, size: u8, message_type: Message, lock: &Arc<Mutex<Vec<ClientAccount>>>, dni_user: &mut String) -> Result<(), std::io::Error> {
    let mut buffer_packet: Vec<u8> = vec![0; size as usize];
    let _aux = stream.read_exact(&mut buffer_packet);
    match message_type {
        Message::Log => {
            let account_credentials = leer_contenido_log(buffer_packet, lock, dni_user);
            if account_credentials.is_ok(){
                send_nice_log_message(stream);
                println!("Hubo un intento de inicio de sesión");
            }
            else{
                send_error_log_message(stream);
            }
        }
        Message::Form => {
            println!("Recibí un formulario");
            let _aux2 = read_forms(buffer_packet, lock, dni_user);
            println!("Intento enviar que el formulario fue recibido");
            send_nice_log_message(stream);
            println!("Se envió que el formulario fue recibido");
        }
        Message::Unknown => {
            println!("No reconozco este mensaje");
        }
        Message::Disconnect => {
            println!("El cliente se desconecto :,C");
            println!("Cierro el stream y el thread");
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        Message::Shutdown => {
            update_database(lock);
            println!("Se actualizó la base de datos");
            println!("Se procede a apagar el servidor!");
            exit(0);
        }
        Message::Appointment => {
            println!("Recibí una consulta de turno");
            let respuesta = get_appointment(&dni_user.as_str(), lock);
            println!("Intento enviar una consulta de turno");
            send_date(stream, respuesta);
            println!("Envié una consulta de turno");
        }
        Message::Delete => {
            println!("Recibí una petición de borrado de cuenta");
            delete_user(dni_user, lock);
        }
        _ => {
            println!("Mensaje desconocido");
        }
    }
    Ok(())
}

pub fn delete_user(dni_user: &mut String, mutex: &Arc<Mutex<Vec<ClientAccount>>>) {
    let mut vector = mutex.lock().unwrap();
    match vector.iter().position(|x| x.get_dni() == Some(dni_user.clone())) {
        Some(x) => {
            vector.remove(x);
        }
        _ => {
            println!("No pude borrar el usuario porque no estaba en la database!");
        }
    }

}

pub fn send_date(stream: &mut TcpStream, respuesta: String) {
    let mut lenght:u8 = 0;
    lenght += (respuesta.len() + 1) as u8;
    let buffer = [Message::Appointment.into(), lenght];
    stream.write_all(&buffer).unwrap();

    let mut buffer_envio: Vec<u8> = Vec::with_capacity(lenght.into());
    buffer_envio.push(respuesta.len() as u8);
    let respuesta_bytes = respuesta.as_bytes();
    for i in 0..respuesta_bytes.len(){
        buffer_envio.push(respuesta_bytes[i]);
    }
    stream.write(&buffer_envio).unwrap();
}