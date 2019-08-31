mod irc;
mod server;
mod tests;

use std::{env, io};
use std::io::{BufRead, Error as IoError, ErrorKind, Write};
use std::net::TcpStream;
use bufstream::BufStream;
use rustyline::error::ReadlineError;
use std::cmp::min;
use crossterm::{terminal, AlternateScreen};
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender, SendError};
use crate::irc::{Message};
use irc::Command::*;
use crate::server::ServerState::*;



#[macro_use]
extern crate strum_macros;


enum ThreadError {
    Io(IoError),
    Channel,
}

impl From<IoError> for ThreadError {
    fn from(err: IoError) -> ThreadError {
        ThreadError::Io(err)
    }
}

impl<T> From<SendError<T>> for ThreadError {
    fn from(_: SendError<T>) -> ThreadError {
        ThreadError::Channel
    }
}

impl From<RecvError> for ThreadError {
    fn from(_: RecvError) -> ThreadError {
        ThreadError::Channel
    }
}

type Thread = JoinHandle<Result<(), ThreadError>>;

struct ServerConnection {
    _thread: Thread
}

impl ServerConnection {
    fn new(server: &str, nick: &str, real_name: &str, logger: Sender<String>) -> ServerConnection {
        let server = String::from(server);
        let nick_msg = format!("NICK {}\r\n", nick);
        let user_msg = format!("USER {} 0 * :{}\r\n", whoami::username(), real_name);

        let join_handle = std::thread::spawn(move || {
            let mut server_state = Disconnected;
            let mut stream = TcpStream::connect(format!("{}:{}", server, 6667))?;
            server_state = Connected;
            stream.write_all(nick_msg.as_bytes()).unwrap();
            stream.write_all(user_msg.as_bytes()).unwrap();
            server_state = LoggingIn;
            let mut stream = BufStream::new(stream);
            loop {
                let mut line = String::new();
                let read_result = stream.read_line(&mut line);
                match read_result {
                    Ok(0) => {
                        logger.send(format!("connection to {} closed!!!", server))?;
                        server_state = Disconnected;
                        break;
                    }
                    Ok(_) => {
                        let parsed = irc::parse_irc_message(&line);
                        match &parsed {
                            Ok(Message { prefix: _, command: Irc("PING"), params }) => {
                                logger.send(format!("PONG!!!"))?;

                                let raw_rest = params.join(" ");
                                let rest_index = raw_rest.find(':').unwrap_or(raw_rest.len());
                                let message = &raw_rest[rest_index..];
                                stream.write_all(format!("PONG {}\r\n", message).as_bytes())?;
                                stream.flush()?;
                            },
                            Ok(Message { prefix: _, command: NumericReply("001"), params }) => {
                                logger.send(format!("Nick {} accepted", params[0]))?;
                                server_state = LoggedIn;
                            },
                            _other => {
//                                logger.send(format!("{:?}", other))?;
                            }
                        }
                        match &parsed {
                            Ok(Message { prefix: _, command: NumericReply("372"), params: _}) => {},
                            Ok(message) => logger.send(format!("OK: \'{:?}\'", message))?,
                            Err(error) => logger.send(format!("cannot parse: {}: \'{}\'", error, line))?
                        }
                    }
                    Err(_) => { logger.send(format!("{}", "unexpected error!!!"))?; }
                }
            }
            return Ok(());
        });

        ServerConnection {
            _thread: join_handle
        }
    }
}

struct Logger {
    _thread: Thread
}

impl Logger {
    fn new(receiver: Receiver<String>, h: u16) -> Logger {
        let join_handle = std::thread::spawn(move || {
            let mut pos = 0;
            loop {
                let message = receiver.recv()?;
                print!("{}", ansi_escapes::CursorSavePosition);
                print!("{}", ansi_escapes::CursorTo::AbsoluteXY(pos, 0));
                println!("{}", message);
                print!("{}", ansi_escapes::CursorRestorePosition);
                std::io::stdout().flush().unwrap();
                pos = min(pos + 1, h - 2);
            }
        });
        Logger {
            _thread: join_handle
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let alternate_screen = AlternateScreen::to_alternate(false)?;
    let terminal = terminal();
    let (_, h) = terminal.terminal_size();
    print!("\x1B[{};{}r", 1, h - 1);
    std::io::stdout().flush().unwrap();

    let (tx, rx) = mpsc::channel();


    match &args[1..] {
        [server, nick, real_name] => {
            let _logger = Logger::new(rx, h);
            let _main_connection = ServerConnection::new(server, nick, real_name, tx);

            let mut rl = rustyline::Editor::<()>::new();

            loop {
                print!("{}", ansi_escapes::CursorTo::AbsoluteXY(h, 0));
                std::io::stdout().flush().unwrap();
                let read_line = rl.readline(">> ");
                match read_line {
                    Ok(_line) => {}
                    Err(ReadlineError::Interrupted) => {
                        break;
                    }
                    Err(_) => println!("No input"),
                }
            }
            alternate_screen.to_main().unwrap();
            return Ok(());
        }
        _ => {
            return Err(IoError::new(ErrorKind::InvalidInput, "Usage: <server> <nick> <real-name>"));
        }
    }
}