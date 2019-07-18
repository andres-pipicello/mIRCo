use std::{env, io};
use std::io::{Error, ErrorKind};
use dns_lookup::get_hostname;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match &args[1..] {
        [server, nick, real_name] => {
            println!(
                "NICK {}\nUSER {} {} {} :{}",
                nick,
                whoami::username(),
                get_hostname().unwrap_or("localhost".to_string()),
                server,
                real_name
            );
            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, "Usage: <server> <nick> <real-name>"));
        }
    }
}
