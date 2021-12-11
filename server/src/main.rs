use std::env::args;
use crate::server::Server;

mod server;
mod configuration;
mod message;
mod file_handling;
mod client_account;
mod logging;
mod appointments;

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
