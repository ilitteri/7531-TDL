use std::fs::File;
use std::path::Path;
use std::fs;
use std::sync::{Arc, Mutex};
use crate::client_account::ClientAccount;
use crate::server::PATH;

pub enum ReadError{
    MissingDNI,
}

pub fn update_database(mutex: &Arc<Mutex<Vec<ClientAccount>>>) {
    match fs::remove_file(PATH) {
        Ok(..) => {
            println!("Éxito al eliminar la database!");
        }
        _ => {
            println!("Problema al eliminar la database!");
        }
    }
    let vector = mutex.lock().unwrap();
    for account in vector.iter(){
        let _aux = write_json(PATH, account.clone());
    }
}
pub fn write_json(path: &str, form:ClientAccount )  -> Result<(), serde_json::Error> {
    let json_file_path = Path::new(path);
    let display = json_file_path.display();

    if json_file_path.exists(){
        let _file = match File::open(&json_file_path) {
            Err(why) => panic!("La información del cliente no pudo ser guardada. Motivo: couldn't open {}: {}", display, why),
            Ok(_file) => _file,
        };
        let data = fs::read_to_string(path).expect("No se pudo leer el archivo");
        let mut clients: Vec<ClientAccount> = Vec::new();
        if fs::metadata(path).unwrap().len() != 0 {
            clients = serde_json::from_str(&data)?;
        }
    
        clients.push(form);
        let json: String = serde_json::to_string(&clients)?;
        fs::write(path, &json).expect("No se pudo escribir el archivo");
    }
         
    else{
        let _file = match File::create(&json_file_path) {
            Err(why) => panic!("La informacion del cliente no pudo ser guardada. Motivo: couldn't create {}: {}", display, why),
            Ok(_file) => _file,
        };
        let mut clients: Vec<ClientAccount> = Vec::new();
        clients.push(form);
        let json: String = serde_json::to_string(&clients)?;
        fs::write(path, &json).expect("No se pudo escribir el archivo");
    }
    Ok(())
}

pub fn get_accounts(path: &str) -> Vec<ClientAccount> {
    let json_file_path = Path::new(path);
    let data = fs::read_to_string(json_file_path).expect("No se pudo leer el archivo");
    let clients: Vec<ClientAccount> = serde_json::from_str(&data).unwrap();
    return clients;
}

pub fn find_user(mutex: &Arc<Mutex<Vec<ClientAccount>>>, dni: String) -> Result<ClientAccount, ReadError> {
    let vector = mutex.lock().unwrap();
    let mut client_account = None;
    for account in vector.iter(){
        if account.get_dni() == Some(dni.clone()){
            client_account = Some(account.clone());
            break;
        }
    }
    match client_account{
        None => Err(ReadError::MissingDNI),
        Some(client) => Ok(client),
    }
}
