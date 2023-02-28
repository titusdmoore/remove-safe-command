use std::env::Args;
use std::time::Instant;
use std::env;
use remove_safe::{command_instance::CommandInstance, instance_path::{PathType}};

fn main() {
    let mut instance: CommandInstance = command_start();

    remove_paths(&mut instance);
    command_end(&instance);
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

    return CommandInstance::new(args, start);
}

fn command_end(instance: &CommandInstance) {
    println!("Command completed with {} files deleted, and {} directories deleted.",
    instance.files_deleted, instance.dirs_deleted);
    println!("In {} milliseconds.", instance.timer.elapsed().as_millis());
}