use super::objects::Command;


pub const COMMAND_TWO: Command = Command {
    name: "COMMAND 2",
    cmd: "c2",
    aliases: Some(&["command2", "second"]),
    description: "Description of command1",
    action: command_two,
};



fn command_two() {
    println!("SECOND COMMAND");
}