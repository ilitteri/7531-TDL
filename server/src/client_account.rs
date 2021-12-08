pub struct ClientAccount {
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub birth_date: Option<String>,
    pub dni: Option<String>,
    pub priority: Option<String>
}

impl ClientAccount {
    pub fn new(name: &str, lastname: &str, email: &str, password: &str, birth_date: &str, dni: &str, priority: &str) -> Self {
        ClientAccount {
            name: Some(name.to_string()),
            lastname: Some(lastname.to_string()),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            birth_date: Some(birth_date.to_string()),
            dni: Some(dni.to_string()),
            priority: Some(priority.to_string())
        }
    }
}