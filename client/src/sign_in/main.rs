use std::{ error, result };

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01_() {
        let name = "Pedro";
        let lastname = "Messi";
        let email = "pmessi@fi.uba.ar";
        let password = "messid10s";
        let birth_date = "13/04/2000";
        let dni = "436463234";

        let account = ClientAccount::new(
            name: name,
            lastname: lastname,
            email: email,
            password: password,
            birth_date: birthdate,
            dni: dni
        );

        assert(account);
    }
}

// impl ClientAccount