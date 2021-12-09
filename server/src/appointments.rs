use crate::client_account::ClientAccount;
use crate::file_handling::get_accounts;
use std::cmp::Ordering;

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

pub fn get_appointment(dni: &str){
    let mut clients: Vec<ClientAccount> = get_accounts("client_data");
    clients.sort();
    println!("{:?}", clients[0]);
    println!("{:?}", clients[1]);
    println!("{:?}", clients[2]);
}