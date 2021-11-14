// use std::{ error, result };

// type TResult<T> = result::Result<T, TError>;
// type TError = Box<dyn error::Error>;

mod client_account;

fn main() {
    println!("Hola");
}

#[cfg(test)]
mod test {
    // use super::*;
    // use std::io;
    use crate::client_account::ClientAccount;

    #[test]
    fn test01_() {
        let name: &str = "Pedro";
        let lastname: &str = "Messi";
        let email: &str = "pmessi@fi.uba.ar";
        let password: &str = "messid10s";
        let birth_date: &str = "13/04/2000";
        let dni: &str = "436463234";

        let account: ClientAccount = ClientAccount::new(
            name,
            lastname,
            email,
            password,
            birth_date,
            dni
        );

        assert_eq!(account.name, name);
        assert_eq!(account.lastname, lastname);
        assert_eq!(account.email, email);
        assert_eq!(account.password, password);
        assert_eq!(account.birth_date, birth_date);
        assert_eq!(account.dni, dni);
    }

    #[test]
    #[should_panic(expected = "missing lastname")]
    fn test02_leaving_lastname_input_text_box_empty_should_raise_an_error() {
        let name = "Pedro";
        let email = "pmessi@fi.uba.ar";
        let lastname = "";
        let password = "messid10s";
        let birth_date = "13/04/2000";
        let dni = "436463234";

        let account = ClientAccount::new(
            name,
            lastname,
            email,
            password,
            birth_date,
            dni
        );

        assert_eq!(account, None);
    }
}