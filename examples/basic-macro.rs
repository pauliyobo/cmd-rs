use cmd_rs::{make_command, CommandProcessor};

#[make_command(help = "testing")]
fn test() -> cmd_rs::Result<()> {
    println!("test");
    Ok(())
}

fn main() {
    CommandProcessor::new()
        .with_prompt(">")
        .add_command(test)
        .run();
}
