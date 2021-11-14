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
                println!("Error al leer!");
                break;
            }
        }
    }
}

// thread::spawn(move || loop {
//     let mut buff = vec![0; MSG_SIZE];
//     match client.read_exact(&mut buff) {
//         Ok(_) => {
//             let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
//             println!("message recv {:?}", msg);
//         },
//         Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
//         Err(_) => {
//             println!("connection with server was severed");
//             break;
//         }
//     }
//
//     match rx.try_recv() {
//         Ok(msg) => {
//             let mut buff = msg.clone().into_bytes();
//             buff.resize(MSG_SIZE, 0);
//             client.write_all(&buff).expect("writing to socket failed");
//             println!("message sent {:?}", msg);
//         },
//         Err(TryRecvError::Empty) => (),
//         Err(TryRecvError::Disconnected) => break
//     }
//
//     thread::sleep(Duration::from_millis(100));
// });