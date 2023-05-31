
// mod commandList;
// use crate::commandList::commandList::commands;

mod objects;
use objects::Command;

mod command_one;
pub use command_one::COMMAND_ONE;
mod command_two;
pub use command_two::COMMAND_TWO;


use crate::color::{red, yellow};

    
pub const COMMANDS: &[Command] = &[
    COMMAND_ONE,
    COMMAND_TWO,
];




pub fn help_command() {
  println!("{}", yellow("---- Help ----"));
  for command in COMMANDS.iter() {

    red(command.name);

    println!("{} - {}", command.name, command.description);
    // (command.action)();
  }
}

pub fn find_command(cmd: &str) -> Option<&Command> {
  for command in COMMANDS.iter() {
    if command.cmd == cmd {
      return Some(command);
    }
  }

  for command in COMMANDS.iter() {
    if let Some(aliases) = command.aliases {
      for alias in aliases.iter() {
        if *alias == cmd {
          return Some(command);
        }
      }
    }
    // for alias in command.aliases.iter(){
    //   if *alias == cmd {
    //     return Some(command);
    //   }
    // }
  }

  println!("{}", red("Command not found"));
  None
}