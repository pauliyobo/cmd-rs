use anyhow::Result;

// The  command trait
pub trait Command<'a> {
    // the command's name 
    fn name(&self) -> &str;
    // the command's help
    fn help(&self) -> Option<&str>;
    // the command's logic which is ran when the command is invoked
    fn execute(&self) -> Result<()>;
}


// A simple command implementation
// all what this does is 
// provide a name and help text
// the execute() method will just print the help text
pub struct Cmd {
    name: String,
    help: String,
}

impl Cmd {
    pub fn new<S: Into<String>>(cmd_name: S, cmd_help: S) -> Self {
        let name = cmd_name.into();
        let help = cmd_help.into();
        Self {
            name,
            help,
        }
    }

}

impl<'a> Command<'a> for Cmd {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn help(&self) -> Option<&str> {
        if self.help.is_empty() {
            return None;
        }
        Some(self.help.as_str())
    }

    fn execute(&self) -> Result<()> {
        println!("{}", self.help().unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let cmd = Cmd::new("test", "testing command");
        assert_eq!(cmd.name, String::from("test"));
        assert_eq!(cmd.help, Some(String::from("testing command")))
    }
}
