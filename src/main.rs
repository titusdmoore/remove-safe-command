use std::collections::HashMap;
use std::env;
use std::env::Args;
use std::path::{PathBuf};

fn main() {
    let args: Args = env::args();
    let instance: CommandInstance = parse_args(args);

    for path in instance.paths {
      // handle remove paths
      println!("{}", path.display());
    }

    
}

// Convert args into usable format
fn parse_args(raw_args: Args) -> CommandInstance {
    let mut settings: HashMap<String, bool> = HashMap::new();
    let mut paths: Vec<PathBuf> = Vec::new();

    for arg in raw_args {
        if arg.starts_with("-") { // Handle Command Args
            let split_args: Vec<&str> = arg.split("").collect();

            for split_arg in split_args {
                match split_arg {
                    "r" => settings.insert("recursive".to_string(), true),
                    "f" => settings.insert("force".to_string(), true),
                    _ => continue,
                };
            }
        } else { // handle paths and invalid parameters
          let path_buf: PathBuf = PathBuf::from(arg);

          if path_buf.exists() {
            paths.push(path_buf);
          } else {
            println!("Invalid Path: Recieved {}", path_buf.display());
          }
        } 
    }

    return CommandInstance {
        settings: settings,
        paths: paths,
    };
}

// Control remove path
fn remove_path(path: PathBuf, settings: HashMap<String, bool>) {

}

struct CommandInstance {
    settings: HashMap<String, bool>,
    paths: Vec<PathBuf>,
}
