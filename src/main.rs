use std::{env, io};
use std::io::{BufRead, Error, ErrorKind, Write};
use std::net::TcpStream;

use bufstream::BufStream;
use rustyline::error::ReadlineError;
use std::cmp::min;
use crossterm::terminal;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    print!("\x1B[?1049h");
    let terminal = terminal();
    let (_, h) = terminal.terminal_size();
    std::io::stdout().flush().unwrap();
    print!("\x1B[{};{}r", 1, h-1);
    std::io::stdout().flush().unwrap();


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
                let mut n = 0;
                stream.write_all(nick_msg.as_bytes()).unwrap();
                stream.write_all(user_msg.as_bytes()).unwrap();
                let mut stream = BufStream::new(stream);
                loop {
                    let pos = min(n, h-2);
                    let mut line = String::new();
                    let read_result = stream.read_line(&mut line);
                    while line.ends_with('\n') | line.ends_with('\r') {
                        line.pop();
                    }
                    print!("{}", ansi_escapes::CursorSavePosition);
                    print!("{}", ansi_escapes::CursorTo::AbsoluteXY(pos, 0));
                    std::io::stdout().flush().unwrap();
                    match read_result {
                        Ok(0) => {
                            println!("connection to {} closed!!!", server);
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
                        Err(_) => println!("{}", "unexpected error!!!")
                    }
                    print!("{}", ansi_escapes::CursorRestorePosition);
                    std::io::stdout().flush().unwrap();
                    n = n + 1;
                }
            });

            let mut rl = rustyline::Editor::<()>::new();

            loop {
                print!("{}", ansi_escapes::CursorTo::AbsoluteXY(h, 0));
                std::io::stdout().flush().unwrap();
                let read_line = rl.readline(">> ");
                match read_line {
                    Ok(line) => println!("Line: {:?}", line),
                    Err(ReadlineError::Interrupted) => {
                        print!("\x1B[?1049l");
                        std::io::stdout().flush().unwrap();
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
