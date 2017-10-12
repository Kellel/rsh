#![feature(io)]
use std::io;
use std::io::{Read, Write};

mod parser;
mod command;


fn main() {
    // Main work loop
    let mut parser = parser::Parser::new();
    for char in io::stdin().chars() {
        match char {
            Ok(c) => {
                match parser.parse(c) {
                    Ok(res) => {
                        match res {
                            Some(cmd) => {
                                print!("{:?}\n", cmd);
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
