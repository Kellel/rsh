use parser;

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
