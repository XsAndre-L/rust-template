use std::io::{self, Write};


mod utils;
use utils::{color::{green, yellow,cyan}, get_command, execute_command};

mod entity;
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
        execute_command(&command, &mut run, commands::MAIN_COMMANDS);
        previouse_command = command;
    }
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




