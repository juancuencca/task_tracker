use std::{env, process};
use task_tracker::Action;

fn main() {
    let action = Action::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing args: {}", e);
        usage();
        process::exit(1);
    });

    println!("Action: {:?}", action);
}

fn usage() {
    eprintln!("\nUsage:"); 
    
    eprintln!("    cargo run -- add <task>");
    eprintln!("    cargo run -- update <id> <task>");
    eprintln!("    cargo run -- delete <id>");
    eprintln!("    cargo run -- mark-in-progress <id>");
    eprintln!("    cargo run -- mark-done <id>");
    eprintln!("    cargo run -- list");
    eprintln!("    cargo run -- list done");
    eprintln!("    cargo run -- list todo");
    eprintln!("    cargo run -- list in-progress\n");
}