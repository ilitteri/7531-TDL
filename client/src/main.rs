mod message;
mod client_account;
mod form;
mod logging;

use std::io;
use std::env::args;
use std::net::TcpStream;
use crate::form::ask_for_form;
use crate::logging::ask_for_log;
use crate::message::{read_date, read_response_from_server, send_consult, send_disconnect, send_log, send_register, send_shutdown};

static CLIENT_ARGS_EXPECTED_LEN: usize = 3;
const OPCION_LOG:i32 = 1;
const OPCION_REGISTER:i32 = 2;
const OPCION_DISCONNECT:i32 = 3;
const OPCION_SECRETA:i32 = 79;
const OPCION_LOG_OFF:i32 = 4;
const OPCION_CONSULT_TURN:i32 = 5;

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
                let log = ask_for_log();
                send_log(&mut stream, &log);
                println!("Envie mi log!");
                //Leo la respuesta del server y tomo una decision
                read_response_from_server(&mut stream);
            }
            OPCION_REGISTER => {
                let form = ask_for_form();
                send_register(&mut stream, &form);
                println!("Envie mi formulario!");
                //Leo la respuesta del server y tomo una decision
                read_response_from_server(&mut stream);
            }
            OPCION_DISCONNECT => {
                send_disconnect(&mut stream);
                println!("Me desconecte!");
                break;
            }
            OPCION_SECRETA => {
                send_shutdown(&mut stream);
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

pub fn logged_menu(stream: &mut TcpStream) {
    loop {
        let mut option_str = String::new();
        println!("Ingresá una opcion:");
        println!("Ingresa 4 para desloguearse!");
        println!("Ingresa 5 para consultar turno!");
        io::stdin()
            .read_line(&mut option_str)
            .expect("Failed to read line");
        let option_string = option_str.trim().to_string();
        let option_int = option_string.parse::<i32>().expect("Error con el parse");

        match option_int {
            OPCION_LOG_OFF => {
                //
                println!("Me desconecte!");
                break;
            }
            OPCION_CONSULT_TURN => {
                send_consult(stream);
                let _aux = read_date(stream); //Manejar
            }
            _=> {
                println!("Opcion invalida intente nuevamente!");
            }
        }
    }

}
