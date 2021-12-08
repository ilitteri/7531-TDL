pub struct AccountCredentials {
    pub dni: Option<String>,
    pub password: Option<String>
}

impl AccountCredentials {
    pub fn new(dni: &str, password: &str) -> Self {
        AccountCredentials {
            password: Some(password.to_string()),
            dni: Some(dni.to_string()),
        }
    }
}