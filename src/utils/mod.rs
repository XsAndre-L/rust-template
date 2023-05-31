use std::io;

pub mod color;
use color::{red, green, yellow, cyan};

// use color::{red, green, yellow, cyan};
use crate::commands::{self, objects::Command};

pub fn get_command () -> String {
  let mut command = String::new();
  io::stdin().read_line(&mut command).expect("> ");
  command
}

pub fn execute_command(command: &str, run: &mut bool, commands: &[Command]) {
  let cmd_str = command.trim();

  if cmd_str == "exit" {
      red("Exiting Program");
      *run = false;
      return;
  }

  if cmd_str == "help" {
      help_command(commands);
      return;
  }

  let cmd = find_command(cmd_str, commands);
  cmd.map(|cmd| (cmd.action)());



  // if cmd_str == "load" {
  //     color::green("Loading");
  //     loading_bar();
  //     reset_line();
  // }


}


pub fn find_command<'a>(cmd: &'a str, COMMANDS: &'a [Command]) -> Option<&'a Command> {
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


pub fn help_command(commands: &[Command]) {
  println!("{}", yellow("---- Help ----"));
  for command in commands.iter() {

    red(command.name);

    println!("{} - {}", command.name, command.description);
    // (command.action)();
  }
}