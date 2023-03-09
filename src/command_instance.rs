use crate::instance_path::{PathType, PathWithStatus};
use std::{collections::HashMap, env::Args, path::PathBuf, time::Instant};

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
        let mut paths: Vec<PathWithStatus> = Vec::new();

        for (index, arg) in raw_args.into_iter().enumerate() {
            // Skip first arg which is the name of the program that ran
            if index == 0 {
                continue;
            }

            if arg.starts_with('-') {
                // Handle Command Args
                let split_args: Vec<&str> = arg.split("").collect();

                for split_arg in split_args {
                    match split_arg {
                        "r" => settings.insert("recursive".to_string(), true),
                        "f" => settings.insert("force".to_string(), true),
                        "q" => settings.insert("quiet".to_string(), true),
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

        CommandInstance {
            settings,
            paths,
            timer,
            // TODO: Implement stats prompt to end command
            files_deleted: 0,
            dirs_deleted: 0,
        }
    }

    pub fn remove_paths(&mut self) {
        for path in self.paths.iter_mut() {
            // Add path remove code - pass in settings
            match path.remove_path(&self.settings) {
                Ok(path_updated) => match path_updated.path_type {
                    PathType::File => {
                        self.files_deleted += 1;
                    }
                    PathType::Dir => {
                        self.dirs_deleted += 1;
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
