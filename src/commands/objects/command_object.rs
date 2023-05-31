pub struct Command {
  pub cmd: &'static str,
  pub aliases: Option<&'static [&'static str]>,
  pub name: &'static str,
  pub description: &'static str,
  pub action: fn(),
}
