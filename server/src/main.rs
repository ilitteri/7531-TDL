use std::env::args;
use crate::server::Server;

mod server;
mod configuration;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != SERVER_ARGS {
        println!("Invalid number of arguments");
        return Err(());
    }
    let server = Server::new(&argv[1]);
    server.run().unwrap();
    Ok(())
}
