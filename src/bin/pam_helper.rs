use pam::Authenticator;
use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let mut authenticator = match Authenticator::with_password("login") {
        Ok(a) => a,
        Err(_) => std::process::exit(1),
    };

    authenticator.get_handler().set_credentials(username.to_string(), password.to_string());

    match authenticator.authenticate() {
        Ok(()) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}