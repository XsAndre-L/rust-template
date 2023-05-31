use super::objects::Command;

use crate::utils::{execute_command, get_command};

pub const FILE_SYSTEM: Command = Command {
    name: "File System",
    cmd: "fs",
    aliases: Some(&["files", "file system"]),
    description: "interact with the file system",
    action: file_system_command,
};

mod fs_commands {
    use super::Command;

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub const LIST: Command = Command {
        name: "List Files",
        cmd: "ls",
        aliases: None,
        description: "Description of command1",
        action: list_files,
    };

    pub const COMMAND_TWO: Command = Command {
        name: "FS COMMAND 2",
        cmd: "c2",
        aliases: Some(&["command2", "second"]),
        description: "Description of command1",
        action: command_two,
    };

    fn list_files() {
        let file_path = Path::new("");
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => panic!("Failed to open file: {}", err),
        };

        // Read the file contents into a string
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("File contents:\n{}", contents);
            }
            Err(err) => {
                panic!("Failed to read file: {}", err);
            }
        }
    }

    fn command_two() {
        println!("SECOND COMMAND");
    }

    pub const FILE_SYSTEM_COMMANDS: &[Command] = &[LIST, COMMAND_TWO];
}

pub fn file_system_command() {
    let mut run = true;

    while run {
        println!("--- File System Command ---");

        let command: String = get_command();
        execute_command(&command, &mut run, fs_commands::FILE_SYSTEM_COMMANDS);
    }
}

// fn navigate_file_system() {
//   // Open the file
//   let file_path = "/";
//   let mut file = match File::open(file_path) {
//       Ok(file) => file,
//       Err(err) => panic!("Failed to open file: {}", err),
//   };

//   // Read the file contents into a string
//   let mut contents = String::new();
//   match file.read_to_string(&mut contents) {
//       Ok(_) => {
//           println!("File contents:\n{}", contents);
//       }
//       Err(err) => {
//           panic!("Failed to read file: {}", err);
//       }
//   }
// }
