#![feature(io)]
use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
#[derive(Clone)]
struct Command {
    cmd: String,
    argv: Vec<String>
}

impl Command {
    fn new() -> Command {
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

struct Parser {
    current: Command,
    buffer: Vec<char>,
    in_progress: bool,
    escape: bool
}

impl Parser {
    fn new() -> Parser {
        Parser{
            current: Command::new(),
            buffer: Vec::new(),
            in_progress: false,
            escape: false
        }
    }

    fn parse(&mut self, input: char) -> Result<Option<Command>,&'static str> {
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
                            return Ok(None)
                        } else {
                            return Ok(Some(self.current.clone()))
                        }
                    } else {
                        self.escape = false;
                    }
                }
                x => {
                    self.buffer.push(x);
                }
            }

        Ok(None)
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

fn main() {
    // Main work loop
    let mut parser = Parser::new();
    for char in io::stdin().chars() {
        match char {
            Ok(c) => {
                match parser.parse(c) {
                    Ok(res) => {
                        match res {
                            Some(cmd) => {
                                print!("{:?}", cmd);
                                io::stdout().flush().unwrap();
                            }
                            None => ()
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        break
                    }
                }
            }
            Err(err) => {
                println!("Read Error: {}", err);
                break
            }
        }
    }
}
