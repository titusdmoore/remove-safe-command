use std::{collections::HashMap, env::Args, time::Instant, path::PathBuf};
use crate::instance_path::PathWithStatus;
 
pub struct CommandInstance {
    pub settings: HashMap<String, bool>,
    pub timer: Instant,
    pub paths: Vec<PathWithStatus>,
    pub files_deleted: i32,
    pub dirs_deleted: i32,
}

impl CommandInstance {
    pub fn new(raw_args: Args, timer: Instant) -> CommandInstance {
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

    pub fn remove_path(&mut self, path: &PathWithStatus) {
        match self.paths.iter().position(|path_iter| path_iter.path.eq(&path.path)) {
            Some(index) =>  {
                self.paths.remove(index);
            }
            None => {}
        }
    }
}
