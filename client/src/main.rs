mod message;

use std::env::args;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::message::{Message, send_disconnect, send_log};

static CLIENT_ARGS_EXPECTED_LEN: usize = 3;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len()  != CLIENT_ARGS_EXPECTED_LEN {
        println!("Invalid number of arguments");
        let app_name = &argv[0];
        println!("{:?} <host> <port>", app_name);
        return Err(());
    }
    let address = argv[1].clone() + ":" + &argv[2];
    println!("Connecting to... {:?}", address);
    client_run(&address).unwrap();
    Ok(())
}

fn client_run(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    // Aca ya se establecio el canal Tcp
    send_log(&mut stream);
    println!("Envie mi log!");

    thread::sleep(Duration::from_millis(1000)); //Sino no termino de recibir y  me desconecto

    send_disconnect(&mut stream);
    println!("Me desconecte!");
    Ok(())
}