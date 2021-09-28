// A  command 

pub struct Command {
    pub name: String,
    pub help: Option<String>,
}

impl Command {
    // Creates a new command, to attach to a prompt
    pub fn new<S: Into<String>>(cmd_name: S, cmd_help: S) -> Command {
        let name = cmd_name.into();
        let help = cmd_help.into();
        Command {
            name: name,
            help: Some(help),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let cmd = Command::new("test", "testing command");
        assert_eq!(cmd.name, String::from("test"));
        assert_eq!(cmd.help, Some(String::from("testing command")))
    }
}