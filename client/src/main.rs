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
        println!("Número de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <port>", app_name);
        return Err(());
    }
    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectando a... {:?}", address);
    client_run(&address).unwrap();
    Ok(())
}

fn client_run(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    loop {
        let mut option_str = String::new();
        println!("Ingresá una opcion:");
        println!("1) Iniciar Sesión");
        println!("2) Registrarse");
        println!("3) Cerrar");
        io::stdin()
            .read_line(&mut option_str)
            .expect("Error al leer la línea");
        let option_string = option_str.trim().to_string();
        let option_int = option_string.parse::<i32>().expect("Error al parsear");

        match option_int {
            OPCION_LOG => {
                let log = ask_for_log();
                send_log(&mut stream, &log);
                read_response_from_server(&mut stream);
            }
            OPCION_REGISTER => {
                let form = ask_for_form();
                send_register(&mut stream, &form);
                read_response_from_server(&mut stream);
            }
            OPCION_DISCONNECT => {
                send_disconnect(&mut stream);
                break;
            }
            OPCION_SECRETA => {
                send_shutdown(&mut stream);
                break;
            }
            _ => {
                println!("Opción equivocada intente nuevamente!");
            }
        }
    }
    Ok(())
}

pub fn logged_menu(stream: &mut TcpStream) {
    loop {
        let mut option_str = String::new();
        println!("Ingresá una opcion:");
        println!("4) Cerrar Sesión");
        println!("5) Consultar Turno");
        io::stdin()
            .read_line(&mut option_str)
            .expect("Failed to read line");
        let option_string = option_str.trim().to_string();
        let option_int = option_string.parse::<i32>().expect("Error al parsear");

        match option_int {
            OPCION_LOG_OFF => {
                println!("\nCerrando sesión...");
                println!("Se cerró sesión correctamente\n");
                break;
            }
            OPCION_CONSULT_TURN => {
                send_consult(stream);
                let _aux = read_date(stream);
            }
            _=> {
                println!("Opcion invalida intente nuevamente!");
            }
        }
    }

}
