use crate::command::Command;
use std::collections::HashMap;
use rustyline::Editor;
use rustyline::error::ReadlineError;

pub struct CommandLoop {
    // the prompt to show each time, such as >>>
    pub prompt: Option<String>,
    // the intro 
    pub intro: Option<String>,
    commands: HashMap<String, Command>,
}

impl CommandLoop {
    pub fn new() -> CommandLoop {
        CommandLoop {
            prompt: None,
            intro: None,
            commands: HashMap::new(),
        }
    }

    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.to_string());
        self
    }

    pub fn with_intro(mut self, intro: &str) -> Self {
        self.intro = Some(intro.to_string());
        self
    }

    pub fn add_command(mut self, cmd: Command) -> Self {
        self.commands.insert(cmd.name.clone(), cmd);
        self
    }

    pub fn run(&self) {
        if let Some(intro) = &self.intro {
            println!("{}", intro);
        }
        let mut rl = Editor::<()>::new();
        loop {
            let readline =  rl.readline(">>");
            match readline {
                Ok(line) => {
                    println!("{}", line);
                },
                _ => (),
            }
        }
    }
}