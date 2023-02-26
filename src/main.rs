use std::collections::HashMap;
use std::env::Args;
use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs, io};

fn main() {
    let mut instance: CommandInstance = command_start();

    remove_paths(&mut instance);
    command_end(&instance);
}

// Convert args into usable format
fn create_command_instance(raw_args: Args, timer: Instant) -> CommandInstance {
    let mut settings: HashMap<String, bool> = HashMap::new();
    // HashMap<PathBuf, IsDeleted (bool)>
    let mut paths: Vec<PathWithStatus> = Vec::new();

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
                paths.push(PathWithStatus {
                    path_type: PathWithStatus::get_path_type(&path_buf),
                    path: path_buf,
                    removed: false,
                });
            } else {
                println!("Invalid Path: Received {}", path_buf.display());
            }
        }
    }

    return CommandInstance {
        settings: settings,
        paths: paths,
        timer: timer,
        // TODO: Implement stats prompt to end command
        files_deleted: 0,
        dirs_deleted: 0,
    };
}

// Control remove path
fn remove_paths(instance: &mut CommandInstance) {
    let settings = &instance.settings;
    println!("Paths len {}", instance.paths.len());
    if instance.paths.len() >= 1 {
        for path in &mut instance.paths {
            match path.remove_path(settings) {
                Ok(returned_path) => {
                    println!("Path removed ok");
                    match &returned_path.path_type {
                        PathType::File => {
                            instance.files_deleted += 1;
                        },
                        PathType::Dir => {
                            instance.dirs_deleted += 1;
                        },
                    }
                },
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn command_start() -> CommandInstance {
    let start = Instant::now();
    let args: Args = env::args();

    return create_command_instance(args, start);
}

fn command_end(instance: &CommandInstance) {
    println!("Command completed with {} files deleted, and {} directories deleted.",
    instance.files_deleted, instance.dirs_deleted);
    println!("In {} milliseconds.", instance.timer.elapsed().as_millis());
}

struct CommandInstance {
    settings: HashMap<String, bool>,
    timer: Instant,
    paths: Vec<PathWithStatus>,
    files_deleted: i32,
    dirs_deleted: i32,
}

struct PathWithStatus {
    path: PathBuf,
    removed: bool,
    path_type: PathType
}

impl PathWithStatus {
    fn remove_path(&mut self, settings: &HashMap<String, bool>) -> Result<&mut PathWithStatus, io::Error> {
        if self.path.is_dir() {
            if settings.contains_key("recursive") && settings.get("recursive") == Some(&true) {
                match fs::remove_dir_all(&self.path) {
                    Ok(_) => {
                        self.removed = true;
                    }
                    Err(e) => return Err(e),
                };
            }
        // Sanity check to ensure that its actually a file, may impact performance, but could prevent errors
        } else if self.path.is_file() {
            // Handle File Delete
            match fs::remove_file(&self.path) {
                Ok(_) => {
                    self.removed = true;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(self)
    }

    fn get_path_type(path: &PathBuf) -> PathType {
        if path.is_dir() {
            return PathType::Dir;
        }

        PathType::File
    }
}

enum PathType {
    File,
    Dir
}