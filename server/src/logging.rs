use crate::client_account::ClientAccount;

const ERROR: u8 = 1;
#[derive(Debug)]
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

pub fn check_credentials(form: AccountCredentials, acc: ClientAccount) -> Result<ClientAccount, u8>{
    if acc.password == form.password{
        return Ok(acc);
    }
    return Err(ERROR);
}
