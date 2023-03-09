use remove_safe::command_instance::CommandInstance;
use std::env;
use std::env::Args;
use std::time::Instant;

fn main() {
    let mut instance: CommandInstance = command_start();

    instance.remove_paths();
    command_end(&instance);
}

fn command_start() -> CommandInstance {
    let start = Instant::now();
    let args: Args = env::args();

    CommandInstance::new(args, start)
}

fn command_end(instance: &CommandInstance) {
    println!(
        "Command completed with {} files deleted, and {} directories deleted.",
        instance.files_deleted, instance.dirs_deleted
    );
    println!("In {} milliseconds.", instance.timer.elapsed().as_millis());
}
