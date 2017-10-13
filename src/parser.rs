use std::io;
use std::io::Read;

#[derive(Debug)]
#[derive(Clone)]
pub struct Command {
    pub cmd: String,
    pub argv: Vec<String>
}

impl Command {
    pub fn new() -> Command {
        Command{
            cmd: String::from(""),
            argv: Vec::new()
        }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        if self.cmd != other.cmd {
            println!("CMD DOESN'T MATCH");
            return false
        }
        if self.argv.len() != other.argv.len() {
            println!("ARGV DOESN'T MATCH");
            return false
        }
        let matches = self.argv.iter().zip(other.argv.iter()).filter(|&(a,b)|a == b).count();
        matches == self.argv.len()
    }
}

pub struct Parser {
    current: Command,
    buffer: Vec<char>,
    in_progress: bool,
    escape: bool
}

pub enum Line {
    Full(Command),
    Partial,
    Empty,
}

impl Parser {
    pub fn new() -> Parser {
        Parser{
            current: Command::new(),
            buffer: Vec::new(),
            in_progress: false,
            escape: false
        }
    }

    pub fn get_command(&mut self) -> Result<Line, &'static str> {
        for character in io::stdin().chars() {
            match self.parse(character.expect("Unable to read from stdin")).expect("Parse error") {
                Line::Full(cmd) => return Ok(Line::Full(cmd)),
                Line::Partial => return Ok(Line::Partial),
                Line::Empty => ()
            }
        }

        return Ok(Line::Empty)
    }

    fn parse(&mut self, input: char) -> Result<Line,&'static str> {
        if self.in_progress == false {
            self.current = Command::new();
        }

        self.in_progress = true;

        match input {
                ' ' => {
                    if self.escape {
                        self.buffer.push(' ');
                        self.escape = false;
                    } else {
                        if self.current.cmd == "" {
                            self.current.cmd = self.buffer.iter().cloned().collect();
                            self.buffer = Vec::new();
                        } else {
                            self.current.argv.push(self.buffer.iter().cloned().collect());
                            self.buffer = Vec::new();
                        }
                    }
                }
                '\\' => {
                    if self.escape {
                        self.buffer.push('\\');
                        self.escape = false;
                    } else {
                        self.escape = true;
                    }
                }
                '\n' => {
                    if ! self.escape {
                        self.in_progress = false;
                        if self.current.cmd == "" {
                            self.current.cmd = self.buffer.iter().cloned().collect();
                            self.buffer = Vec::new();
                        } else {
                            self.current.argv.push(self.buffer.iter().cloned().collect());
                            self.buffer = Vec::new();
                        }
                        if self.current.cmd == "" {
                            return Ok(Line::Empty)
                        } else {
                            return Ok(Line::Full(self.current.clone()))
                        }
                    } else {
                        self.escape = false;
						return Ok(Line::Partial)
                    }
                }
                x => {
                    self.buffer.push(x);
                }
            }

        Ok(Line::Empty)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn basic_parser_test() {
        let input = String::from("echo hello world");
        let expected = Command{
            cmd: String::from("echo"),
            argv: vec!(String::from("hello"), String::from("world"))
        };


        let mut parser = Parser::new();
        for c in input.chars() {
            parser.parse(c).unwrap();
        }

        let result = parser.parse('\n').unwrap().unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn newline_parser_test() {
        let input = String::from("echo hello world");
        let other_input = String::from("foo");
        let expected = Command{
            cmd: String::from("echo"),
            argv: vec!(String::from("hello"), String::from("worldfoo"))
        };

        let mut parser = Parser::new();
        for c in input.chars() {
            parser.parse(c).unwrap();
        }

        parser.parse('\\').unwrap();
        parser.parse('\n').unwrap();

        for c in other_input.chars() {
            parser.parse(c).unwrap();
        }

        let result = parser.parse('\n').unwrap().unwrap();
        assert_eq!(result, expected)
    }
}
