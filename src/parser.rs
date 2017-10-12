use command;

pub struct Parser {
    current: command::Command,
    buffer: Vec<char>,
    in_progress: bool,
    escape: bool
}

impl Parser {
    pub fn new() -> Parser {
        Parser{
            current: command::Command::new(),
            buffer: Vec::new(),
            in_progress: false,
            escape: false
        }
    }

    pub fn parse(&mut self, input: char) -> Result<Option<command::Command>,&'static str> {
        if self.in_progress == false {
            self.current = command::Command::new();
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
        let expected = command::Command{
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
        let expected = command::Command{
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
