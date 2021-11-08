use std::collections::HashMap;

const DEFAULT_PORT: u16 = 7666;
const DEFAULT_DUMPFILE: &str = "dump.txt";
const DEFAULT_LOGFILE: &str = "logfile.txt";
const DEFAULT_IP: &str = "127.0.0.1";

pub struct Configuration {
    port: u16,
    dumpfile: String,
    logfile: String,
    ip: String,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            port: DEFAULT_PORT,
            dumpfile: DEFAULT_DUMPFILE.to_string(),
            logfile: DEFAULT_LOGFILE.to_string(),
            ip: DEFAULT_IP.to_string(),
        }
    }

    pub fn set_config(&mut self, file_path: &str) -> Result<bool, String> {
        let map;
        match self.parse(file_path) {
            Ok(map_) => map = map_,
            Err(err) => return Err(err),
        }
        if let Some(err) = self.set_all_params(map) {
            return Err(err);
        }
        Ok(true)
    }

    fn parse(&mut self, file_path: &str) -> Result<HashMap<String, String>, String> {
        let file: String = match std::fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(_) => return Err("Error when trying to open the file".to_string()),
        };
        let mut map: HashMap<String, String> = HashMap::new();
        let lines = file.lines();

        for line in lines {
            let name_and_value: Vec<&str> = line.split('=').collect();
            let config_name: String = name_and_value[0]
                .to_lowercase()
                .replace(' ', "")
                .to_string();
            let value: String = name_and_value[1].replace(' ', "").to_string();
            map.insert(config_name, value);
        }
        Ok(map)
    }

    fn set_all_params(&mut self, map: HashMap<String, String>) -> Option<String> {
        if let Some(logfile_) = map.get("logfile") {
            self.logfile = logfile_.to_string();
            println!("Loaded log file : {}", self.logfile);
        }
        if let Some(port_) = map.get("port") {
            self.port = port_.parse().unwrap();
        }
        if let Some(dumpfile_) = map.get("dumpfile") {
            self.dumpfile = dumpfile_.to_string();
            println!("Uploaded archive configuration : {}", self.dumpfile);
        }
        None
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn get_log_file(&self) -> String {
        self.logfile.to_string()
    }
}