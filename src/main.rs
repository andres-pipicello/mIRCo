use std::{env, io};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use rustyline::error::ReadlineError;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match &args[1..] {
        [server, nick, real_name] => {
            let nick_msg = format!("NICK {}\r\n", nick);
            let user_msg = format!("USER {} 0 * :{}\r\n",
                                   whoami::username(),
                                   real_name
            );
            let mut stream = TcpStream::connect(format!("{}:{}", server, 6667))?;
            let _join_handle = std::thread::spawn(move || {
                stream.write_all(nick_msg.as_bytes()).unwrap();
                stream.write_all(user_msg.as_bytes()).unwrap();
                let mut stream = stream;
                loop {
                    let mut buffer = [0u8; 512];
                    let read = stream.read(&mut buffer).unwrap();
                    println!("{}", String::from_utf8_lossy(&buffer[..read]));
                }
            });

            let mut rl = rustyline::Editor::<()>::new();

            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => println!("Line: {:?}", line),
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(_) => println!("No input"),
                }
            }
            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, "Usage: <server> <nick> <real-name>"));
        }
    }
}
