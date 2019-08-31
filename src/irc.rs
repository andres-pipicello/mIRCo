use lazy_static::lazy_static;
use regex::bytes::Regex;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: Command<'a>,
    pub params: Vec<&'a str>,
}

#[derive(Display, Debug, PartialEq)]
pub enum Command<'a> {
    Irc(&'a str),
    NumericReply(&'a str),
}

#[derive(Display, Debug)]
pub enum ParseError {
    DelimitersMissing,
    EmptyMessage,
    NoCommand,
}

fn is_three_digit_number(command: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("[0-9][0-9][0-9]").unwrap();
    }
    RE.is_match(command.as_bytes())
}

pub fn parse_irc_message(line: &str) -> Result<Message, ParseError> {
    let len = line.len();
    if len < 2 {
        return Err(ParseError::DelimitersMissing);
    }
    if &line[len - 2..len] != "\r\n" {
        return Err(ParseError::DelimitersMissing);
    }
    let actual_message = &line[0..len - 2];
    let mut parts = actual_message.split_ascii_whitespace();
    if let Some(first) = parts.next() {
        let (prefix, command) = if first.starts_with(':') {
            (Some(&first[1..]), parts.next())
        } else {
            (None, Some(first))
        };

        return match command {
            Some(command) => {
                let command = if is_three_digit_number(command) {
                    Command::NumericReply(command)
                } else {
                    Command::Irc(command)
                };
                Ok(Message {
                    prefix,
                    command,
                    params: parts.collect(),
                })
            }
            None => Err(ParseError::NoCommand),
        };
    } else {
        return Err(ParseError::EmptyMessage);
    }
}

#[cfg(test)]
mod tests {
    use crate::irc::*;

    #[test]
    fn test_ping() {
        assert_eq!(
            parse_irc_message("PING :xDyDFIN~[Z\r\n").unwrap(),
            Message {
                prefix: None,
                command: Command::Irc("PING"),
                params: vec!(":xDyDFIN~[Z")
            }
        );
    }
    #[test]
    fn test_notice() {
        assert_eq!(
            parse_irc_message(":1a4229d69393.example.com NOTICE * :*** Looking up your hostname...\r\n").unwrap(),
            Message {
                prefix: Some("1a4229d69393.example.com"),
                command: Command::Irc("NOTICE"),
                params: vec!("*", ":***", "Looking", "up", "your", "hostname...")
            }
        );
    }
}
