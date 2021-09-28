# Cmd-rs
Cmd-rs is an attempt to build a library which helps to build interactive commandline apps with ease.  
It was inspired by the cmd module in python and the excellent [cmd2](https://github.com/python-cmd2/cmd2) library.  
## Usage
To use this library you can set the dependency which points to this git repository, since it has not been published on cargo yet.  

```toml
[dependencies]
cmd-rs = { git = "https://github.com/pauliyobo/cmd-rs"}
```

It is possible to use the `Command` trait to define a command which will then be assigned to the command processor.

```rust
use cmd_rs::{Command, CommandProcessor, Result};

// A simple command
struct SimpleCommand;

// implement the Command trait
impl<'a>  Command<'a> for SimpleCommand {
    // the name of the command
    fn name(&self) -> &str {
        "simplecommand"
    }
    
    
    // the help text, which can be optional
    fn help(&self) -> Option<&str> {
        None
    }
    
    // the method that will be invoked when the command is called
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("{}", self.name());
        Ok(())
    }
}

fn main() {
    CommandProcessor::new()
        // the intro message which will be displayed when the processor is started
        .with_intro("Simple intro")
        .with_prompt("simple>")
        .add_command(SimpleCommand)
        .run();
}
```

There is also another example in the examples directory.
## Roadmap
I started building cmd-rs also to gain more experience with the rust programming language, so The API  will most likely break a lot. Sorry for the inconvenience. However those are  some of the steps I'd like to get done at some point, in no particular order.
* [ ] Adding validation when calling commands. E.G. the library should error if a command contains whitespace.
* [ ] Adding more documentation 
* [ ] Even if this probably at a later stage. Adding a CI pipeline. perhaps github actions?
* [ ] add pre-commit hook to lint and format code instead of having to do that manually
* [ ] Adding tests where possible
* [ ] publishing on cargo
## Contributing
If you find this project useful and would like to improve it, feel free to open an issue or send a pull request