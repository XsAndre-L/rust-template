
// mod commandList;
// use crate::commandList::commandList::commands;

pub mod objects;
use objects::Command;

mod command_one;
pub use command_one::COMMAND_ONE;
mod command_two;
pub use command_two::COMMAND_TWO;
mod fs;
pub use fs::FILE_SYSTEM;


// use crate::color::{red, yellow};
// use crate::utils::color::{red, yellow};
use crate::utils::color::{red, yellow};

    
pub const MAIN_COMMANDS: &[Command] = &[
    COMMAND_ONE,
    COMMAND_TWO,
    FILE_SYSTEM,
];






// pub fn find_command(cmd: &str) -> Option<&Command> {
//   for command in COMMANDS.iter() {
//     if command.cmd == cmd {
//       return Some(command);
//     }
//   }

//   for command in COMMANDS.iter() {
//     if let Some(aliases) = command.aliases {
//       for alias in aliases.iter() {
//         if *alias == cmd {
//           return Some(command);
//         }
//       }
//     }
//     // for alias in command.aliases.iter(){
//     //   if *alias == cmd {
//     //     return Some(command);
//     //   }
//     // }
//   }

//   println!("{}", red("Command not found"));
//   None
// }