use std::{collections::HashMap, time::Instant};

pub struct CommandInstance {
    settings: HashMap<String, bool>,
    timer: Instant,
    paths: Vec<PathWithStatus>,
    files_deleted: i32,
    dirs_deleted: i32,
}