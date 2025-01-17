use chrono::prelude::*;
use std::fmt;
use crate::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    fn new(id: usize, description: impl Into<String>) -> Self {
        Task { 
            id, 
            description: description.into(), 
            status: Status::Todo, 
            created_at: Local::now(), 
            updated_at: Local::now(), 
        }
    }

    fn update(&mut self, description: impl Into<String>, status: Status) {
        self.description = description.into();
        self.status = status;
        self.updated_at = Local::now();
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task {{\n")?;
        write!(f, "  Id: {}\n", self.id)?;
        write!(f, "  Description: {}\n", self.description)?;
        write!(f, "  Status: {:?}\n", self.status)?;
        write!(f, "  Created at: {}\n", self.created_at)?;
        write!(f, "  Updated at: {:?}\n", self.updated_at)?;
        write!(f, "}};")
    }
}

#[derive(Debug, Default)]
pub struct TaskManager {
    tasks: Vec<Task>,
    last_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, description: impl Into<String>) {
        let new_task = Task::new(self.last_id+1, description);
        self.tasks.push(new_task);
        self.last_id += 1;
    }

    pub fn update(&mut self, id: usize, description: Option<impl Into<String>>, status: Option<Status>) -> Result<()> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            match (description, status) {
                (Some(description), Some(status)) => task.update(description, status),
                (None, Some(status)) => task.update(task.description.clone(), status),
                (Some(description), None) => task.update(description, task.status.clone()),
                (None, None) => task.update(task.description.clone(), task.status.clone())
            }
            Ok(())
        } else {
            Err("task does not exist".into())
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<()> {
        if self.tasks.iter().any(|task| task.id == id) {
            self.tasks.retain(|task| task.id != id);
            Ok(())
        } else {
            Err("task does not exist".into())
        }
    }

    pub fn list(&self, status: Option<Status>) {
        match status {
            Some(Status::Todo) => {
                for task in self.tasks.iter().filter(|task| task.status == Status::Todo) {
                    println!("{:?}", task);
                }
            }, 
            Some(Status::InProgress) => {
                for task in self.tasks.iter().filter(|task| task.status == Status::InProgress) {
                    println!("{:?}", task);
                }
            }, 
            Some(Status::Done) => {
                for task in self.tasks.iter().filter(|task| task.status == Status::Done) {
                    println!("{:?}", task);
                }
            }, 
            None => {
                for task in self.tasks.iter() {
                    println!("{:#?}", task);
                }
            }, 

        }
    } 
}