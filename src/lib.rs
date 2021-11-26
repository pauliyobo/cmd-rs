pub extern crate inventory;

pub mod command;
mod command_processor;

pub use self::command::RegisteredCommand;
pub use self::command_processor::CommandProcessor;
pub use inventory as _inventory;
pub use anyhow::Result;
pub use cmd_rs_macro::make_command;
