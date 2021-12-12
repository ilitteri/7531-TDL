use crate::server::Server;
use std::env::args;

mod appointments;
mod client_account;
mod configuration;
mod file_handling;
mod logging;
mod message;
mod server;

static SERVER_ARGS: usize = 2;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Número inválido de argumentos");
        return Err(());
    }
    let server = Server::new(&argv[1]);
    server.run().unwrap();
    Ok(())
}
