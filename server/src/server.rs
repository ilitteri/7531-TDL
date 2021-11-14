use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use crate::configuration::Configuration;
use crate::message::read_message;

pub struct Server {
    //
    cfg: Configuration,
    //
}

impl Server {
    pub fn new(file_path: &str) -> Self {
        let mut config = Configuration::new();
        let _aux = config.set_config(file_path); //Manejar
        Server {
            cfg: config,
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let address = self.cfg.get_address();
        println!("IP: {}", &address);
        Server::wait_new_clients(&address);
        Ok(())
    }

    fn wait_new_clients(address: &str) -> std::io::Result<()> {
        let mut index: usize = 0;
        loop {
            let listener = TcpListener::bind(&address)?;
            let connection: (TcpStream, SocketAddr) = listener.accept()?;
            let mut client_stream = connection.0;
            thread::Builder::new().name("Client-Listener".into()).spawn(move || {
                println!("Se lanzo un cliente!.");
                handle_client(index, &mut client_stream);
            }).unwrap();
            index += 1;
        }
    }
}

fn handle_client(id :usize, stream: &mut TcpStream) {
    let mut stream_cloned = stream.try_clone().unwrap();
    read_packet_from_client(&mut stream_cloned);
}

fn read_packet_from_client(stream: &mut TcpStream) {
    loop {
        let mut num_buffer = [0u8; 2]; //Recibimos 2 bytes
        match stream.read_exact(&mut num_buffer) {
            Ok(_) => {
                let message_type = num_buffer[0].into();
                read_message(stream, num_buffer[1], message_type);
            }
            Err(_) => {
                println!("El cliente se desconecto y cerro el stream.");
                break;
            }
        }
    }
}
