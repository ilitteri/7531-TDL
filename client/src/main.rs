mod message;
mod client_account;
mod form;
mod logging;

use std::io;
use std::env::args;
use std::net::TcpStream;
use crate::form::ask_for_form;
use crate::logging::ask_for_log;
use crate::message::{send_disconnect, send_log, send_register};

static CLIENT_ARGS_EXPECTED_LEN: usize = 3;
const OPCION_LOG:i32 = 1;
const OPCION_REGISTER:i32 = 2;
const OPCION_DISCONNECT:i32 = 3;

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
    //
    loop {
        let mut option_str = String::new();
        println!("Ingresá una opcion:");
        println!("Ingresa 1 para logearte!");
        println!("Ingresa 2 para registrarte!");
        println!("Ingresa 3 para desconectarte!");
        io::stdin()
            .read_line(&mut option_str)
            .expect("Failed to read line");
        let option_string = option_str.trim().to_string();
        let option_int = option_string.parse::<i32>().expect("Error con el parse");

        match option_int {
            OPCION_LOG => {
                let _log = ask_for_log();
                send_log(&mut stream);
                println!("Envie mi log!");
            }
            OPCION_REGISTER => {
                let form = ask_for_form();
                send_register(&mut stream, &form);
                println!("Envie mi formulario!");
            }
            OPCION_DISCONNECT => {
                send_disconnect(&mut stream);
                println!("Me desconecte!");
                break;
            }
            _ => {
                println!("Opcion equivocada intente nuevamente!");
            }
        }
    }
    Ok(())
}