use anyhow::Result;
use cmd_rs::command::Command;
use cmd_rs::prompt::CommandLoop;

// A simple command implementation
// all what this does is 
// provide a name and help text
// the execute() method will just print the help text
struct Cmd {
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

    fn execute(&self, args: &[&str]) -> Result<()> {
        println!("{}", self.help().unwrap());
        Ok(())
    }
}

fn main() {
    let cmd = CommandLoop::new()
        .with_intro("test intro")
        .with_prompt(">")
        .add_command(Cmd::new("test", "test command"))
        .run();
}
