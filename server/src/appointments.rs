use crate::client_account::ClientAccount;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

impl Ord for ClientAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority).cmp(&(other.priority))
    }
}

impl PartialOrd for ClientAccount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ClientAccount {
    fn eq(&self, other: &Self) -> bool {
        (self.priority) == (other.priority)
    }
}

impl Eq for ClientAccount { }

pub fn get_appointment(dni: &str, mutex: &Arc<Mutex<Vec<ClientAccount>>>) -> String{
    let mut clients = mutex.lock().unwrap();
    let mut indice:i16 = -1;
    let mut contador = 0;
    clients.sort();

    for account in clients.iter(){
        println!("El dni en el vector es -> {}", account.get_dni().unwrap());
        println!("El dni buscado es -> {}", dni.to_string().clone());
        if account.get_dni() == Some(dni.to_string().clone()){
            indice = contador;
            break;
        }
        println!("Itere! {}", contador);
        contador += 1;
    }

    return indice.to_string();
}