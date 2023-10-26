use super::connect_options;
use super::module;

#[derive(Debug)]
pub enum Command {
   Goto(module::Module),
   Connect(connect_options::ConnectOptions),
   Wait,
}

impl Command {
   pub fn to_string(&self) -> String {
      match self {
         Command::Goto(module) => format!("GOTO {}", module.as_str()),
         Command::Connect(options) => format!("CONNECT {}", options.to_string()),
         _ => "WAIT".to_owned(),
      }
   }
}
