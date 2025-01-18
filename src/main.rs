use std::{
    env, 
    process, 
    fs::{self, File}, 
    io::{self, Write, Read}
};
use task_tracker::{Action, Tracker};

pub fn create_file_write_all(file_path: &str, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;

    Ok(())
}

pub fn read_file_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    
    Ok(file_contents)
}

fn main() {
    let action = Action::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing args: \"{}\"", e);
        eprintln!("Usage:"); 
        eprintln!("    cargo run add <task>");
        eprintln!("    cargo run update <id> <task>");
        eprintln!("    cargo run delete <id>");
        eprintln!("    cargo run mark-in-progress <id>");
        eprintln!("    cargo run mark-done <id>");
        eprintln!("    cargo run list");
        eprintln!("    cargo run list done");
        eprintln!("    cargo run list todo");
        eprintln!("    cargo run list in-progress\n");
        process::exit(1);
    });

    let file_name = "tasks.json";

    if fs::exists(file_name).expect("Unexpected error checking existence") {
        let file_content = read_file_string(file_name).unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        });

        let mut tracker: Tracker = serde_json::from_str(&file_content).unwrap_or_else(|e| {
            eprintln!("Couldn't parse json to struct: {}", e);
            process::exit(1);
        });

        tracker.process_action(action).unwrap_or_else(|| {
            eprintln!("Task id does not exist.");
            process::exit(1);
        });

        let json = serde_json::to_string(&tracker).unwrap_or_else(|e| {
            eprintln!("Couldn't parse struct to json: {}", e);
            process::exit(1);
        });

        create_file_write_all(file_name, json.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error while creating and writing file: {}", e);
            process::exit(1);
        });
    } else {
        let mut tracker = Tracker::new();
        
        tracker.process_action(action).unwrap_or_else(|| {
            eprintln!("Task id does not exist.");
            process::exit(1);
        });

        let json = serde_json::to_string(&tracker).unwrap_or_else(|e| {
            eprintln!("Couldn't parse struct to json: {}", e);
            process::exit(1);
        });

        create_file_write_all(file_name, json.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Error while creating and writing file: {}", e);
            process::exit(1);
        });
    }
}