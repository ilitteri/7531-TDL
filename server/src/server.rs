use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::client_account::ClientAccount;
use crate::configuration::Configuration;
use crate::file_handling::get_accounts;
use crate::message::read_message;

pub struct Server {
    cfg: Configuration,
}

pub static PATH: &str = "client_data";

impl Server {
    pub fn new(file_path: &str) -> Self {
        let mut config = Configuration::new();
        let _aux = config.set_config(file_path);
        Server {
            cfg: config,
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let address = self.cfg.get_address();
        println!("IP: {}", &address);
        let _aux = Server::wait_new_clients(&address);
        Ok(())
    }

    fn wait_new_clients(address: &str) -> std::io::Result<()> {
        let mut index: usize = 0;
        let clients = get_accounts(PATH);
        let lock_clients = Arc::new(Mutex::new(clients));
        loop {
            let mutex_clone = Arc::clone(&lock_clients);
            let listener = TcpListener::bind(&address)?;
            let connection: (TcpStream, SocketAddr) = listener.accept()?;
            let mut client_stream = connection.0;
            thread::Builder::new().name("Client-Listener".into()).spawn(move || {
                println!("Se lanzo un cliente!.");
                handle_client(index, &mut client_stream, &mutex_clone);
            }).unwrap();
            index += 1;
        }
    }
}

fn handle_client(_id :usize, stream: &mut TcpStream, lock: &Arc<Mutex<Vec<ClientAccount>>>) {
    let mut stream_cloned = stream.try_clone().unwrap();
    read_packet_from_client(&mut stream_cloned, lock);
}

fn read_packet_from_client(stream: &mut TcpStream, lock: &Arc<Mutex<Vec<ClientAccount>>>) {
    let mut dni_user: String = String::new();
    loop {
        let mut num_buffer = [0u8; 2];
        match stream.read_exact(&mut num_buffer) {
            Ok(_) => {
                let message_type = num_buffer[0].into();
                let _aux = read_message(stream, num_buffer[1], message_type, lock, &mut dni_user);
            }
            Err(_) => {
                println!("El cliente se desconecto y cerro el stream.");
                break;
            }
        }
    }
}
