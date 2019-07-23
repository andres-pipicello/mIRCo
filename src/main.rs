use std::{env, io};
use std::io::{BufRead, Error, ErrorKind, Write};
use std::net::TcpStream;

use bufstream::BufStream;
use rustyline::error::ReadlineError;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match &args[1..] {
        [server, nick, real_name] => {
            let server = server.clone();
            let nick_msg = format!("NICK {}\r\n", nick);
            let user_msg = format!("USER {} 0 * :{}\r\n",
                                   whoami::username(),
                                   real_name
            );
            let mut stream = TcpStream::connect(format!("{}:{}", server, 6667))?;
            let _join_handle = std::thread::spawn(move || {
                stream.write_all(nick_msg.as_bytes()).unwrap();
                stream.write_all(user_msg.as_bytes()).unwrap();
                let mut stream = BufStream::new(stream);
                loop {
                    let mut line = String::new();
                    let read_result = stream.read_line(&mut line);
                    match read_result {
                        Ok(0) => {
                            eprintln!("connection to {} closed!!!", server);
                            break;
                        }
                        Ok(_) => {
                            let pos = line.find(|x: char| x.is_ascii_whitespace()).unwrap_or(line.len());
                            match &line[0..pos] {
                                "PING" => {
                                    println!("PONG!!!");
                                    let raw_rest = &line[pos..];
                                    let rest_index = raw_rest.find(':').unwrap_or(raw_rest.len());
                                    let message = &raw_rest[rest_index..];
                                    stream.write_all(format!("PONG :{}\r\n", message).as_bytes()).unwrap();
                                    stream.flush().unwrap();
                                }
                                _ => println!("unhandled message {}", line)
                            }
                        }
                        Err(_) => eprintln!("{}", "unexpected error!!!")
                    }
                }
            });

            let mut rl = rustyline::Editor::<()>::new();

            loop {
                let read_line = rl.readline(">> ");
                match read_line {
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
