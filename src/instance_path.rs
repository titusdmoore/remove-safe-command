use crossbeam_channel::{self, Receiver, Sender};
use std::{
    collections::HashMap,
    fs::{self},
    io::{self, Error, ErrorKind},
    path::PathBuf,
    thread::{self},
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
                let mut child_paths = self.path.get_path_children()?;

                // Check to see if enough to do multi-threaded
                if child_paths.len() > 4 {
                
                    // Create Channel for communicating that the thread has completed the path removal
                    let (thread_complete_tx_1, thread_complete_rx): (
                        Sender<String>,
                        Receiver<String>,
                    ) = crossbeam_channel::unbounded();
                    let thread_complete_tx_2 = thread_complete_tx_1.clone();
                    let thread_complete_tx_3 = thread_complete_tx_1.clone();
                    let thread_complete_tx_4 = thread_complete_tx_1.clone();

                    // Create Channels for sending paths to threads
                    let (path_channel_tx_1, path_channel_rx_1): (
                        Sender<PathBuf>,
                        Receiver<PathBuf>,
                    ) = crossbeam_channel::unbounded();
                    let (path_channel_tx_2, path_channel_rx_2): (
                        Sender<PathBuf>,
                        Receiver<PathBuf>,
                    ) = crossbeam_channel::unbounded();
                    let (path_channel_tx_3, path_channel_rx_3): (
                        Sender<PathBuf>,
                        Receiver<PathBuf>,
                    ) = crossbeam_channel::unbounded();
                    let (path_channel_tx_4, path_channel_rx_4): (
                        Sender<PathBuf>,
                        Receiver<PathBuf>,
                    ) = crossbeam_channel::unbounded();

                    // Create Threads to remove
                    let thread_handle_1 = thread::Builder::new()
                        .name("thread1".to_string())
                        .spawn(move || {
                            for path_to_remove in path_channel_rx_1 {
                                let thread_name = match thread::current().name() {
                                    Some(name) => String::from(name),
                                    None => String::from(""),
                                };

                                if path_to_remove.is_dir() {
                                    match fs::remove_dir_all(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => print!("{}", e),
                                    }
                                } else if path_to_remove.is_file() {
                                    match fs::remove_file(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => println!("{}", e),
                                    }
                                }

                                match thread_complete_tx_1.send(thread_name) {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", e),
                                }
                            }
                        })?;

                    let thread_handle_2 = thread::Builder::new()
                        .name("thread2".to_string())
                        .spawn(move || {
                            for path_to_remove in path_channel_rx_2 {
                                let thread_name = match thread::current().name() {
                                    Some(name) => String::from(name),
                                    None => String::from(""),
                                };

                                if path_to_remove.is_dir() {
                                    match fs::remove_dir_all(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => print!("{}", e),
                                    }
                                } else if path_to_remove.is_file() {
                                    match fs::remove_file(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => println!("{}", e),
                                    }
                                }

                                match thread_complete_tx_2.send(thread_name) {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", e),
                                }
                            }
                        })?;

                    let thread_handle_3 = thread::Builder::new()
                        .name("thread3".to_string())
                        .spawn(move || {
                            for path_to_remove in path_channel_rx_3 {
                                let thread_name = match thread::current().name() {
                                    Some(name) => String::from(name),
                                    None => String::from(""),
                                };

                                if path_to_remove.is_dir() {
                                    match fs::remove_dir_all(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => print!("{}", e),
                                    }
                                } else if path_to_remove.is_file() {
                                    match fs::remove_file(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => println!("{}", e),
                                    }
                                }

                                match thread_complete_tx_3.send(thread_name) {
                                    Ok(_) => {},
                                    Err(e) => println!("{}", e)
                                }
                            }
                        })?;

                    let thread_handle_4 = thread::Builder::new()
                        .name("thread4".to_string())
                        .spawn(move || {
                            for path_to_remove in path_channel_rx_4 {
                                let thread_name = match thread::current().name() {
                                    Some(name) => String::from(name),
                                    None => String::from(""),
                                };

                                if path_to_remove.is_dir() {
                                    match fs::remove_dir_all(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => print!("{}", e),
                                    }
                                } else if path_to_remove.is_file() {
                                    match fs::remove_file(path_to_remove) {
                                        Ok(_) => {}
                                        Err(e) => println!("{}", e),
                                    }
                                }

                                match thread_complete_tx_4.send(thread_name) {
                                    Ok(_) => {},
                                    Err(e) => println!("{}", e)
                                }
                            }
                        })?;

                    // Handle initial file population of the four threads
                    for index in 1..=4 {
                        if let Some(child_path) = child_paths.pop() {
                            match index {
                                // TODO: better handle errors here
                                1 => {
                                    path_channel_tx_1.send(child_path).ok();
                                }
                                2 => {
                                    path_channel_tx_2.send(child_path).ok();
                                }
                                3 => {
                                    path_channel_tx_3.send(child_path).ok();
                                }
                                4 => {
                                    path_channel_tx_4.send(child_path).ok();
                                }
                                _ => {}
                            }
                        }
                    }

                    let controller_handle = thread::spawn(move || {
                        for path_completed in thread_complete_rx {
                            if let Some(child_path) = child_paths.pop() {
                                match path_completed.as_ref() {
                                    "thread1" => {
                                        path_channel_tx_1.send(child_path).ok();
                                    }
                                    "thread2" => {
                                        path_channel_tx_2.send(child_path).ok();
                                    }
                                    "thread3" => {
                                        path_channel_tx_3.send(child_path).ok();
                                    }
                                    "thread4" => {
                                        path_channel_tx_4.send(child_path).ok();
                                    }
                                    _ => {}
                                }
                            } else {
                                // No more paths to give, break
                                break;
                            }
                        }
                    });
                    controller_handle.join().unwrap();
                    thread_handle_1.join().unwrap();
                    thread_handle_2.join().unwrap();
                    thread_handle_3.join().unwrap();
                    thread_handle_4.join().unwrap();

                    fs::remove_dir_all(&self.path).unwrap();
                } else {
                    match fs::remove_dir_all(&self.path) {
                        Ok(_) => {
                            self.removed = true;
                        }
                        Err(e) => return Err(e),
                    };
                }
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
