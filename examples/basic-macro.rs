use cmd_rs::{make_command, CommandProcessor, Result};

#[make_command(help = "testing")]
fn test() -> Result<()> {
    println!("test");
    Ok(())
}

fn main() {
    CommandProcessor::new()
        .with_prompt(">")
        .add_command(test)
        .run();
}
