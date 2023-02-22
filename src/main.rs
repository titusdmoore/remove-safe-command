use std::env;
use std::env::Args;
use std::collections::HashMap;

struct CommandInstance {
  settings: HashMap<String, bool>,
  paths: Vec<String>
}

fn main() {
  let args: Args = env::args();

  
}

// Convert args into usable format
fn parse_args(raw_args: Args) -> CommandInstance {
  let mut settings: HashMap<String, bool> = HashMap::new();
  let mut paths: Vec<String> = Vec::new();

  for arg in raw_args {
    if arg.starts_with("-") {
      // Handle settings
    } // handle paths and invalid parameters
  }

  return CommandInstance { 
    settings: settings, 
    paths: paths 
  };
}