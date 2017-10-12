
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
