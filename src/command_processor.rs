use crate::command::{Command, RegisteredCommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct CommandProcessor {
    /// the prompt symbol or text  to show each time, such as >>>
    prompt: Option<String>,
    /// the intro message that will be printed on startup
    intro: Option<String>,
    /// the exit message, printed before the shutdown of the application
    exit: Option<String>,
}

impl Default for CommandProcessor {
    fn default() -> Self {
        Self {
            prompt: None,
            intro: None,
            exit: None,
        }
    }
}

impl CommandProcessor {
    pub fn new() -> Self {
        Default::default()
    }

    fn maybe_get_intro(&self) -> Option<&str> {
        if let Some(intro) = &self.intro {
            return Some(intro.as_str());
        }
        None
    }

    fn get_prompt(&self) -> &str {
        if let Some(prompt) = &self.prompt {
            prompt
        } else {
            ""
        }
    }

    pub fn maybe_get_command(&self, name: &str) -> Option<&RegisteredCommand> {
        for command in inventory::iter::<RegisteredCommand> {
            if command.name() == name {
                return Some(command);
            }
        }
        return None;
    }

    pub fn maybe_get_exit(&self) -> Option<&str> {
        if let Some(exit) = &self.exit {
            return Some(exit.as_str());
        }
        None
    }

    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.to_string());
        self
    }

    pub fn with_intro(mut self, intro: &str) -> Self {
        self.intro = Some(intro.to_string());
        self
    }

    pub fn with_exit(mut self, exit: &str) -> Self {
        self.exit = Some(exit.to_string());
        self
    }

    pub fn run(&self) {
        if let Some(intro) = self.maybe_get_intro() {
            println!("{}", intro);
        }
        let mut rl = Editor::<()>::new();
        loop {
            let readline = rl.readline(self.get_prompt());
            match readline {
                Ok(line) => {
                    self.feed(&line);
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                _ => (),
            }
        }
        if let Some(exit) = self.maybe_get_exit() {
            println!("{}", exit);
        }
    }

    fn feed(&self, line: &str) {
        let mut cmd_and_args = line.split_whitespace();
        let cmd_text = cmd_and_args.next().unwrap();
        let args: Vec<&str> = cmd_and_args.collect();
        match self.maybe_get_command(cmd_text) {
            Some(cmd) => {
                cmd.execute(&args).unwrap();
            }
            None => {
                println!("No command found");
            }
        }
    }
}
