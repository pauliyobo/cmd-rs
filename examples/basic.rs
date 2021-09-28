use cmd_rs::command::Command;
use cmd_rs::prompt::CommandLoop;

fn main() {
    let cmd = CommandLoop::new()
        .with_intro("test intro")
        .with_prompt(">")
        .add_command(Command::new("test", "test command"))
        .run();
}
