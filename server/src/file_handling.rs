use std::fs::File;
use std::path::Path;
//use std::io::Write;
//use std::io::Read;
//use std::fs::OpenOptions;
use std::fs;
//use serde_json::{Deserializer, Value};
use crate::client_account::ClientAccount;

pub enum ReadError{         //cambiar esto del enum
    MissingDNI,
}

/*#fn main() -> std::io::Result<()> {
   let vec = vec![1, 2, 3];
    let file = File::create("a")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &vec)?;
    writer.flush()?;
    Ok(())
}*/
  

// TODO: ESCRIBIR JSON
pub fn write_json(path: &str, form:ClientAccount )  -> Result<(), serde_json::Error> {
    let json_file_path = Path::new(path);
    let display = json_file_path.display();

    if json_file_path.exists(){
        let file = match File::open(&json_file_path) {  //saque el mut
            Err(why) => panic!("La informacion del cliente no pudo ser guardada. Motivo: couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mut clients: Vec<ClientAccount> = Vec::new();
        if fs::metadata(path).unwrap().len() != 0 {
            clients = serde_json::from_str(&data)?;
        }
    
        clients.push(form);
        let json: String = serde_json::to_string(&clients)?;
        fs::write(path, &json).expect("Unable to write file");
    }
         
    else{
        let file = match File::create(&json_file_path) {    //saque el mut
            Err(why) => panic!("La informacion del cliente no pudo ser guardada. Motivo: couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        let mut clients: Vec<ClientAccount> = Vec::new();
        clients.push(form);
        let json: String = serde_json::to_string(&clients)?;
        fs::write(path, &json).expect("Unable to write file");
    }
    Ok(())
}

pub fn get_accounts(path: &str) -> Vec<ClientAccount> {
    let json_file_path = Path::new(path);
    let data = fs::read_to_string(json_file_path).expect("Unable to read file");
    let clients: Vec<ClientAccount> = serde_json::from_str(&data).unwrap();
    return clients;
}

pub fn read_json(path: &str, dni: String) -> Result<ClientAccount, ReadError> {

    let clients: Vec<ClientAccount> = get_accounts(path);
    let mut client_account = None;
    for account in clients{
        if account.dni == Some(dni.clone()){
            client_account = Some(account);
            break;
        }
    }
    match client_account{
        None => Err(ReadError::MissingDNI),
        Some(client) => Ok(client),
    }
}
