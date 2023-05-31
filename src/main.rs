use std::io::{self, Write};

// use crate::color::Tred;
// use crate::color::Tgreen;

use crate::color::{red, green, yellow, cyan};

mod entity;
mod color;
mod commands;

// Entry point of the program
fn main() {

    let mut run = true;
    let mut previouse_command = String::new();
    // let ent = entity::Entity::new(1, String::from("test"));
    // ent.do_something();

    while run {
        // println!("Previouse Command, {}",previouse_command);
        let command: String = get_command();
        execute_command(&command, &mut run);
        previouse_command = command;
    }
}

fn get_command () -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("> ");
    command
}

fn execute_command(command: &str, run: &mut bool) {
    let cmd_str = command.trim();

    if cmd_str == "exit" {
        color::red("Exiting Program");
        *run = false;
        return;
    }

    if cmd_str == "help" {
        commands::help_command();
        return;
    }

    let cmd = commands::find_command(cmd_str);
    cmd.map(|cmd| (cmd.action)());



    // if cmd_str == "load" {
    //     color::green("Loading");
    //     loading_bar();
    //     reset_line();
    // }


}

fn loading_bar() {
    print!("\x1b[1;32m[");
    io::stdout().flush().unwrap(); // Flush the output buffer
    for i in 0..50 {
        print!("\x1b[1;32m=");
        io::stdout().flush().unwrap(); // Flush the output buffer
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    print!("\x1b[1;32m] \x1b[0m");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    reset_line();
}

fn reset_line() {
    print!("\x1b[2K\r"); 
}




