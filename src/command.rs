use crate::Result;

// The  command trait
pub trait Command {
    // the command's name
    fn name(&self) -> &str;
    // the command's help
    fn help(&self) -> Option<&str>;
    // the command's logic which is ran when the command is invoked
    fn execute(&self, args: &[&str]) -> Result<()>;
}

// A registered command
pub struct RegisteredCommand(Box<dyn Command>);

impl RegisteredCommand {
    pub fn new(cmd: Box<dyn Command>) -> Self {
        RegisteredCommand(cmd)
    }
}

impl Command for RegisteredCommand {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn help(&self) -> Option<&str> {
        self.0.help()
    }

    fn execute(&self, _args: &[&str]) -> Result<()> {
        self.0.execute(_args)
    }
}
inventory::collect!(RegisteredCommand);
