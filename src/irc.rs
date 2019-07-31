use regex::bytes::Regex;
use lazy_static::lazy_static;


#[derive(Debug)]
pub struct Message<'a> {
    pub prefix: Option<&'a str>,
    pub command: Command<'a>,
    pub params: Vec<&'a str>,
}

#[derive(Display, Debug)]
pub enum Command<'a> {
    IRC(&'a str),
    TDN(&'a str),
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
    let mut whitespace = actual_message.split_ascii_whitespace();
    if let Some(first) = whitespace.next() {
        let (prefix, command) = if first.starts_with(':') {
            (Some(&first[1..]), whitespace.next())
        } else {
            (None, Some(first))
        };

        return match command {
            Some(command) => {
                let command = if is_three_digit_number(command) {Command::TDN(command)} else {Command::IRC(command)};
                Ok(Message { prefix, command, params: whitespace.collect() })
            },
            None => Err(ParseError::NoCommand)
        };
    } else {
        return Err(ParseError::EmptyMessage);
    }
}
