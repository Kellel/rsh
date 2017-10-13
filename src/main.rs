#![feature(io)]
use std::io;
use std::io::Write;
use std::process::Command;

mod parser;
mod builtins;

struct Rsh {
    builtin_manager: builtins::BuiltinManager,
    parser: parser::Parser
}

impl Rsh {
    fn new() -> Rsh {
        Rsh {
            builtin_manager: builtins::BuiltinManager::new(),
            parser: parser::Parser::new()
        }
    }

    fn run_command(&self, cmd: parser::Command) {
        if self.builtin_manager.is_builtin(cmd.cmd.clone()) {
            self.builtin_manager.run(cmd).unwrap();
            return;
        }
        let mut command = Command::new(cmd.cmd)
            .args(cmd.argv)
            .spawn()
            .expect("Failed exec command");
        let ecode = command.wait().expect("failed to wait");
        assert!(ecode.success());
    }
}

fn main() {
    let mut rsh = Rsh::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    loop {
        match rsh.parser.get_command().expect("Error parsing command") {
            parser::Line::Full(cmd) => {
                rsh.run_command(cmd);
                print!("$ ");
                io::stdout().flush().unwrap();
            }
            parser::Line::Partial => {
                print!("> ");
                io::stdout().flush().unwrap();
            }
            parser::Line::Empty => ()
        }
    }
}
