# Task Tracker 

Task tracker is a command line interface used to track and manage tasks. Track what you need to do, what you have done, and what you are currently working on. 
Task tracker is inspired by the [Roadmap.sh guidelines on task tracker CLI](https://roadmap.sh/projects/task-tracker)

## Quick Start

Download or clone this repository on your device. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install), then execute the program with the following command:

```bash
$ cargo run -- add "Go for a walk"
$ cargo run -- list
```

## Features
With this tool the user is able to:

- Add, Update, and Delete tasks
- Mark a task as in progress or done
- List all tasks
- List all tasks that are done
- List all tasks that are not done
- List all tasks that are in progress

## Example 
The list of commands and their usage is given below:
```bash
// Adding a new task
cargo run -- add "Buy groceries"

// Updating and deleting tasks
cargo run -- update 1 "Buy groceries and cook dinner"
cargo run -- delete 1

// Marking a task as in progress or done
cargo run -- mark-in-progress 1
cargo run -- mark-done 1

// Listing all tasks
cargo run -- list

// Listing tasks by status
cargo run -- list done
cargo run -- list todo
cargo run -- list in-progress
```
