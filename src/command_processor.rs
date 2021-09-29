use crate::command::{Command, RegisteredCommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;
//use std::collections::HashMap;

pub struct CommandProcessor {
    // the prompt to show each time, such as >>>
    pub prompt: Option<String>,
    // the intro message
    pub intro: Option<String>,
    // commands: HashMap<String, Box<dyn Command<'a>>>,
}

impl Default for CommandProcessor {
    fn default() -> Self {
        Self {
            prompt: None,
            intro: None,
            // commands: HashMap::new(),
        }
    }
}

impl CommandProcessor {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_prompt(&self) -> &str {
        if let Some(prompt) = &self.prompt {
            prompt
        } else {
            ""
        }
    }

    pub fn get_command(&self, name: &str) -> Option<&RegisteredCommand> {
        for command in inventory::iter::<RegisteredCommand> {
            if command.name() == name {
                return Some(command);
            }
        }
        return None;
    }

    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.to_string());
        self
    }

    pub fn with_intro(mut self, intro: &str) -> Self {
        self.intro = Some(intro.to_string());
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
                    let mut cmd_and_args = line.split_whitespace();
                    let cmd_text = cmd_and_args.next().unwrap();
                    let args: Vec<&str> = cmd_and_args.collect();
                    match self.get_command(cmd_text) {
                        Some(cmd) => {
                            cmd.execute(&args).unwrap();
                        }
                        None => {
                            println!("No command found");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                _ => (),
            }
        }
    }
}
