use anyhow::Result;

// The  command trait
pub trait Command<'a> {
    // the command's name
    fn name(&self) -> &str;
    // the command's help
    fn help(&self) -> Option<&str>;
    // the command's logic which is ran when the command is invoked
    fn execute(&self, args: &[&str]) -> Result<()>;
}
