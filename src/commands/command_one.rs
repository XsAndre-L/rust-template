use super::objects::Command;

use super::super::utils::color::red;


pub const COMMAND_ONE: Command = Command {
    name: "COMMAND 1",
    cmd: "c1",
    aliases: None,
    description: "Description of command1",
    action: command_one,
};


fn command_one() {
  println!("{}", red("FIRST COMMAND"));
}