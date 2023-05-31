use std::mem::replace;



pub fn red(text: &str) -> String {
  attach_color(text, "\x1b[1;31m")
}
pub fn green(text: &str) -> String {
  attach_color(text, "\x1b[1;32m")
}
pub fn yellow(text: &str) -> String {
  attach_color(text, "\x1b[1;33m")
}
pub fn cyan(text: &str) -> String {
  attach_color(text, "\x1b[1;46m")
}

fn attach_color(text: &str, color: &str) -> String {
  let mut result = String::from(color);
  result.push_str(text);
  result.push_str("\x1b[0m");
  result
}

// pub fn Tred(text: &str) -> String {
//     let mut result = String::from("\x1b[1;31m");
//     result.push_str(text);
//     result.push_str("\x1b[0m");
//     result
//   // print!("\x1b[1;31m{}\x1b[0m", text)
// }
// pub fn Tgreen(text: &str) -> String {
//   let mut result = String::from("\x1b[1;32m");
//     result.push_str(text);
//     result.push_str("\x1b[0m");
//     result
//   // print!("\x1b[1;32m{}\x1b[0m", text)
// }

pub fn print_reset(text: &str) {
  println!("\x1b[0m{}\x1b[0m", text)
}
pub fn print_bold(text: &str) {
  println!("\x1b[1m{}\x1b[0m", text)
}
pub fn print_underline(text: &str) {
  println!("\x1b[4m{}\x1b[0m", text)
}
pub fn print_inverse(text: &str) {
  println!("\x1b[7m{}\x1b[0m", text)
}