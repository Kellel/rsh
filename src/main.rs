#![feature(io)]
use std::io;
use std::io::Write;
use std::process::Command;

mod parser;

fn run_command(cmd: parser::Command) {
    let mut command = Command::new(cmd.cmd)
        .args(cmd.argv)
        .spawn()
        .expect("Failed exec command");
    let ecode = command.wait().expect("failed to wait");
    assert!(ecode.success());
}

fn main() {
    // Main work loop
    let mut parser = parser::Parser::new();

    print!("$ ");
    io::stdout().flush().unwrap();

    loop {
        match parser.get_command().expect("Error parsing command") {
            parser::Line::Full(cmd) => {
                run_command(cmd);
                io::stdout().flush().unwrap();
                print!("$ ");
            }
            parser::Line::Partial => {
                print!("> ");
                io::stdout().flush().unwrap();
            }
            parser::Line::Empty => ()
        }
    }
}
