use crate::command::Command;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

pub struct CommandLoop {
    // the prompt to show each time, such as >>>
    pub prompt: Option<String>,
    // the intro
    pub intro: Option<String>,
    commands: HashMap<String, Command>,
}

impl Default for CommandLoop {
    fn default() -> Self {
        CommandLoop {
            prompt: None,
            intro: None,
            commands: HashMap::new(),
        }
    }
}

impl CommandLoop {
    pub fn new() -> CommandLoop {
        Default::default()
    }

    fn get_prompt(&self) -> &str {
        if let Some(prompt) = &self.prompt {
            prompt
        } else {
            ""
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
            let readline = rl.readline(self.get_prompt());
            match readline {
                Ok(line) => {
                    println!("{}", line);
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                _ => (),
            }
        }
    }
}
