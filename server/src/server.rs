use crate::configuration::Configuration;

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
}