pub struct ClientAccount {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
    pub dni: String
}

impl ClientAccount {
    pub fn new(name: &str, lastname: &str, email: &str, password: &str, birth_date: &str, dni: &str) -> Self {
        ClientAccount {
            name: name.to_string(),
            lastname: lastname.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            birth_date: birth_date.to_string(),
            dni: dni.to_string()
        }
    }
}

// struct MissingFieldError {
//     description: String
// }
//
// impl MissingLastnameError for MissingFieldError {
//     fn description(&self) -> &str {
//         &self.description;
//     }
// }