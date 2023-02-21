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

  return CommandInstance { 
    settings: settings, 
    paths: paths 
  };
}