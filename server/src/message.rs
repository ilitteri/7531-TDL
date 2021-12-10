use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex};

use crate::client_account::ClientAccount;
use crate::file_handling::{find_user, write_json};
use crate::appointments::get_appointment;
use crate::logging::check_credentials;
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
    Appointment
}

impl From<u8> for Message {
    fn from(code :u8) -> Message {
        match code & 0xF0 {
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

fn leer_contenido_formulario(buffer_packet: Vec<u8>, mutex: &Arc<Mutex<Vec<ClientAccount>>>, dni_user: &mut String) -> Result<u8, u8> {
    let mut _index = 0 as usize;
    let mut _dni: Option<String> = None;
    let dni_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _dni = Some(bytes2string(&buffer_packet[_index..(_index + dni_size)])?);
    _index += dni_size;
    let aux = _dni.clone().unwrap();
    *dni_user = aux;

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
    let _aux = write_json("client_data", client_account); //Manejar

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
    //println!("El Dni es -> {}", dni.unwrap());

    let mut _password: Option<String> = None;
    let password_size: usize = buffer_packet[(_index) as usize] as usize;
    _index += 1 as usize;
    _password = Some(bytes2string(&buffer_packet[_index..(_index + password_size)])?);
    _index += password_size;

    //println!("El password es -> {}", password.unwrap());

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

//fn make_log(log: AccountCredentials) ->ClientAccount

pub fn read_message(stream: &mut TcpStream, size: u8, message_type: Message, lock: &Arc<Mutex<Vec<ClientAccount>>>, dni_user: &mut String) -> Result<(), std::io::Error> {
    let mut buffer_packet: Vec<u8> = vec![0; size as usize];
    let _aux = stream.read_exact(&mut buffer_packet); //Manejar
    match message_type {
        Message::Log => {
            println!("Recibi un intento de log!");
            let account_credentials = leer_contenido_log(buffer_packet, lock, dni_user); // Manejar
            println!("El dni despues del log es -> {}", dni_user);
            //let client_account = make_log(account_credentials);
            if account_credentials.is_ok(){
                send_nice_log_message(stream);
                println!("Se intenta realizar un log");
            }
            else{
                send_error_log_message(stream);
            }
        }
        Message::Form => {
            println!("Recibi un formulario!");
            let _aux2 = leer_contenido_formulario(buffer_packet, lock, dni_user); // Manejar
            // Aca cuando se tenga la base de clientes se analiza es un usuario correcto o no y se avisa al usuario
            send_nice_log_message(stream);
            println!("Envie que fue exitoso el form!");
        }
        Message::Unknown => {
            println!("No se que paso!");
        }
        Message::Disconnect => {
            println!("El cliente se desconecto :,C!");
            println!("Cierro el stream y el thread!");
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        Message::Shutdown => {


            println!("Se procede a apagar el servidor!");
            //Me guardo todas las historias medicas q tenga del archivo de donde la lei y salgo
            exit(0);
        }
        Message::Appointment => {
            println!("Me llego una consulta de turno!");
            println!("El dni antes del get appoitment es -> {}", dni_user);
            let respuesta = get_appointment(&dni_user.as_str(), lock);
            println!("Le mande en cuantos dias tienen el turno! ({})", respuesta);
            send_date(stream, respuesta);
        }
        _ => {
            println!("Unknown message!")
        }
    }
    Ok(())
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