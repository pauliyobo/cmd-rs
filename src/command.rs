use crate::Result;

/// The  `Command` trait
/// This trait  defines a command that will be invoked and executed by the `CommandProcessor`
/// while this trait is public
/// it's use is discouraged
/// prefer using the `make_command` macro instead
pub trait Command {
    // the command's name
    fn name(&self) -> &str;
    // the command's help text, which can be empty
    fn help(&self) -> Option<&str>;
    // the command's logic which is ran when the command is invoked
    fn execute(&self, args: &[&str]) -> Result<()>;
}

/// A registered command
/// While this struct isn't used directly
/// it is used by the `inventory`  when using the `make_command` macro
/// and it just holds a reference to an actual  command
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
