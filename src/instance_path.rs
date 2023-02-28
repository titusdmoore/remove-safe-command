use std::{path::PathBuf, collections::HashMap, io, fs};

pub struct PathWithStatus {
    pub path: PathBuf,
    pub removed: bool,
    pub path_type: PathType,
}

impl PathWithStatus {
    pub fn remove_path(&mut self, settings: &HashMap<String, bool>) -> Result<&mut PathWithStatus, io::Error> {
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

    pub fn get_path_type(path: &PathBuf) -> PathType {
        if path.is_dir() {
            return PathType::Dir;
        }

        PathType::File
    }
}

pub enum PathType {
    File,
    Dir
}