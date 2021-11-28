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

    pub fn get_dni(&self) -> Option<String> {
        return self.dni.clone();
    }

    pub fn get_birth_date(&self) -> Option<String> {
        return self.birth_date.clone();
    }

    pub fn get_password(&self) -> Option<String> {
        return self.password.clone();
    }

    pub fn get_email(&self) -> Option<String> {
        return self.email.clone();
    }

    pub fn get_lastname(&self) -> Option<String> {
        return self.lastname.clone();
    }

    pub fn get_name(&self) -> Option<String> {
        return self.name.clone();
    }

    pub fn get_priority(&self) -> Option<String> {
        return self.priority.clone();
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