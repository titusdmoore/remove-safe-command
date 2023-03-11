use std::{
    collections::HashMap,
    fs::{self},
    io::{self, Error, ErrorKind},
    path::PathBuf,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub struct PathWithStatus {
    pub path: PathBuf,
    pub removed: bool,
    pub path_type: PathType,
}

impl PathWithStatus {
    pub fn remove_path(
        &mut self,
        settings: &HashMap<String, bool>,
    ) -> Result<&mut PathWithStatus, io::Error> {
        if self.path.is_dir() {
            if settings.contains_key("recursive") && settings.get("recursive") == Some(&true) {
                let child_paths = self.path.get_path_children()?;

                // Check to see if enough to do multi-threaded
                if child_paths.len() > 4 {
                    child_paths.par_iter().for_each(|path| {
                        if path.is_dir() {
                            fs::remove_dir_all(path).unwrap();
                        } else {
                            fs::remove_file(path).unwrap();
                        }
                    });
                }
                
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
    Dir,
}

impl WithPathChildren for PathBuf {
    fn get_path_children(&self) -> Result<Vec<PathBuf>, Error> {
        if self.is_dir() {
            let mut paths: Vec<PathBuf> = Vec::new();

            for child_read_dir in self.read_dir()? {
                let child_read_dir = child_read_dir?;

                paths.push(child_read_dir.path().to_path_buf());
            }

            return Ok(paths);
        }

        Err(Error::new(
            ErrorKind::Other,
            "Path provided is not a directory",
        ))
    }
}

trait WithPathChildren {
    fn get_path_children(&self) -> Result<Vec<PathBuf>, Error>;
}
