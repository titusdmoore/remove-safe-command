use crossbeam_channel::{self, Receiver, Sender};
use std::{
    collections::HashMap,
    fs::{self, read_dir},
    io::{self, Error, ErrorKind},
    path::PathBuf,
    thread,
};

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
                    // Create Channel for communicating that the thread has completed the path removal
                    let (thread_complete_tx_1, thread_complete_rx): (Sender<String>, Receiver<String>) =
                        crossbeam_channel::unbounded();
                    let thread_complete_tx_2 = thread_complete_tx_1.clone();
                    let thread_complete_tx_3 = thread_complete_tx_1.clone();
                    let thread_complete_tx_4 = thread_complete_tx_1.clone();

                    // Create Channels for sending paths to threads
                    let (path_channel_tx_1, path_channel_rx_1): (Sender<&PathBuf>, Receiver<&PathBuf>) = crossbeam_channel::unbounded();
                    let (path_channel_tx_2, path_channel_rx_2): (Sender<&PathBuf>, Receiver<&PathBuf>) = crossbeam_channel::unbounded();
                    let (path_channel_tx_3, path_channel_rx_3): (Sender<&PathBuf>, Receiver<&PathBuf>) = crossbeam_channel::unbounded();
                    let (path_channel_tx_4, path_channel_rx_4): (Sender<&PathBuf>, Receiver<&PathBuf>) = crossbeam_channel::unbounded();


                    /*
                      Potential Solution to having to repopulate a thread whenever it completes deleting a path
                      Create a controller thread (that gets the vec of the paths sent to it) that receives the name of the thread that completed (thread_complete_rx) then
                      sends the next path to be deleted, or despawns the thread if no more paths, then from main wait for the controller thread to finish, then continue
                     */

                    // Create Threads to remove
                    let thread_handle_1 =
                        thread::Builder::new()
                            .name("thread1".to_string())
                            .spawn(move || {
                              let path_to_remove = path_channel_rx_1.recv().unwrap();

                              if path_to_remove.is_dir() {
                                fs::remove_dir_all(path_to_remove);
                              } else if path_to_remove.is_file() {
                                fs::remove_file(path_to_remove);
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

        return Err(Error::new(
            ErrorKind::Other,
            "Path provided is not a directory",
        ));
    }
}

trait WithPathChildren {
    fn get_path_children(&self) -> Result<Vec<PathBuf>, Error>;
}
