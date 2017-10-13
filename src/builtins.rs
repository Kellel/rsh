use parser;

use std::env;
use std::path;

struct Builtin {
    name: &'static str,
    func: fn(cmd: parser::Command) -> Result<(),&'static str>
}

pub struct BuiltinManager {
    builtins: Vec<Builtin>
}

impl BuiltinManager {
    pub fn new() -> BuiltinManager {
        BuiltinManager {
            builtins: vec!(
                Builtin{
                    name: "test",
                    func: builtin_test
                },
                Builtin{
                    name: "cd",
                    func: builtin_cd
                }
            )
        }
    }

    pub fn is_builtin(&self, cmd: String) -> bool {
        for builtin in self.builtins.iter() {
            if builtin.name == cmd {
                return true
            }
        }
        return false
    }
    pub fn run(&self, cmd: parser::Command) -> Result<(), &'static str> {
        for builtin in self.builtins.iter() {
            if builtin.name == cmd.cmd {
                let func = builtin.func;
                return func(cmd);
            }
        }
        return Err("Command not found")
    }
}

fn builtin_test (cmd: parser::Command) -> Result<(),&'static str> {
    println!("TEST: {:?}", cmd.argv);
    Ok(())
}

fn builtin_cd (cmd: parser::Command) -> Result<(), &'static str> {
    match cmd.argv.len() {
        0 => {
            let home = env::home_dir().expect("Home dir not found");
            match env::set_current_dir(&home) {
                Ok(_) => Ok(()),
                Err(_) => Err("Unable to cd to home")
            }
        }
        1 => {
            let path = path::Path::new(&cmd.argv[0]);
            if path.is_relative() {
                let current = env::current_dir().expect("Failed to get PWD");
				match env::set_current_dir(&current.join(path)) {
					Ok(_) => Ok(()),
					Err(_) => Err("Unable to cd to relative path")
				}
            } else {
				match env::set_current_dir(&path) {
					Ok(_) => Ok(()),
					Err(_) => Err("Unable to cd to relative path")
				}
            }
        }
        _ => Err("Too many arguments")
    }
}
