use std::collections::HashMap;
use std::env;
use std::env::Args;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Args = env::args();
    let instance: CommandInstance = parse_args(args);

    for path in &instance.paths {
        // handle remove paths
        remove_path(path, &instance);
    }
}

// Convert args into usable format
fn parse_args(raw_args: Args) -> CommandInstance {
    let mut settings: HashMap<String, bool> = HashMap::new();
    let mut paths: Vec<PathBuf> = Vec::new();

    for arg in raw_args {
        if arg.starts_with("-") {
            // Handle Command Args
            let split_args: Vec<&str> = arg.split("").collect();

            for split_arg in split_args {
                match split_arg {
                    // TODO: Add Verbose Flag
                    "r" => settings.insert("recursive".to_string(), true),
                    "f" => settings.insert("force".to_string(), true),
                    _ => continue,
                };
            }
        } else {
            // handle paths and invalid parameters
            let path_buf: PathBuf = PathBuf::from(arg);

            if path_buf.exists() {
                paths.push(path_buf);
            } else {
                println!("Invalid Path: Received {}", path_buf.display());
            }
        }
    }

    return CommandInstance {
        settings: settings,
        paths: paths,
        // TODO: Implement stats prompt to end command
        files_deleted: 0,
        dirs_deleted: 0,
    };
}

// Control remove path
fn remove_path(path: &PathBuf, instance: &CommandInstance) {
    let settings = &instance.settings;
    // Check if file or dir
    if path.is_dir() {
        if settings.contains_key("recursive") && settings.get("recursive") == Some(&true) {
            println!("removed {}", path.display());
            match fs::remove_dir_all(path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    // Sanity check to ensure that its actually a file, may impact performance, but could prevent errors
    } else if path.is_file() {
        // Handle File Delete
        match fs::remove_file(path) {
          Ok(_) => {}
          Err(e) => {
            println!("{}", e);
          }
        }
    }
}

struct CommandInstance {
    settings: HashMap<String, bool>,
    paths: Vec<PathBuf>,
    files_deleted: i32,
    dirs_deleted: i32,
}
