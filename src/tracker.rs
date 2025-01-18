use chrono::prelude::*;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::Action;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    fn new(id: usize, description: impl Into<String>) -> Self {
        let now = Local::now();

        Task { 
            id, 
            description: description.into(), 
            status: Status::Todo, 
            created_at: now, 
            updated_at: now, 
        }
    }

    fn with_status(&mut self, status: Status) {
        self.status = status;
        self.with_updated_at();
    }
    
    fn with_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
        self.with_updated_at();
    }

    fn with_updated_at(&mut self) {
        self.updated_at = Local::now();
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task: {}\n", self.description)?;
        write!(f, "    Id: {}\n", self.id)?;
        write!(f, "    Status: {:?}\n", self.status)?;
        write!(f, "    Created: {}\n", self.created_at.format("%d/%m/%Y %H:%M:%S"))?;
        write!(f, "    Updated: {}\n", self.updated_at.format("%d/%m/%Y %H:%M:%S"))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tracker {
    tasks: Vec<Task>,
    last_id: usize,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker::default()
    }

    fn add(&mut self, description: impl Into<String>) {
        let next_id = self.last_id + 1;
        let task = Task::new(next_id, description);
        self.tasks.push(task);
        self.last_id = next_id;
    }
    
    fn update(&mut self, id: usize, description: impl Into<String>) -> Option<()> {
        let task = self.tasks.iter_mut().find(|task| task.id == id)?;
        task.with_description(description);
        
        Some(())
    }
    
    fn mark_status(&mut self, id: usize, status: Status) -> Option<()> {
        let task = self.tasks.iter_mut().find(|task| task.id == id)?;
        task.with_status(status);
        
        Some(())
    }

    fn delete(&mut self, id: usize) -> Option<()> {
        if self.tasks.iter().any(|task| task.id == id) {
            self.tasks.retain(|task| task.id != id);
            Some(())
        } else {
            None
        }
    }

    fn print<'a, T: Iterator<Item = &'a Task>>(&self, tasks: T) {
        for task in tasks {
            println!("{}", task);
        }
    }

    fn list(&self, status: Option<Status>) {
        use Status::*;
        let iter = self.tasks.iter();

        match status {
            Some(Todo) => self.print(iter.filter(|task| task.status == Todo)),
            Some(InProgress) => self.print(iter.filter(|task| task.status == InProgress)),
            Some(Done) => self.print(iter.filter(|task| task.status == Done)),
            None => self.print(iter),
        }
    }

    pub fn process_action(&mut self, action: Action) -> Option<()> {
        use Action::*;
        use crate::Status::*;

        match action {
            Add(description) => self.add(description),
            Update{ id, description } => self.update(id, description)?, 
            Delete(id) => self.delete(id)?, 
            MarkInProgress(id) => self.mark_status(id, InProgress)?,
            MarkDone(id) => self.mark_status(id, Done)?,
            List(None) => self.list(None),
            List(Some(Todo)) => self.list(Some(Todo)),
            List(Some(InProgress)) => self.list(Some(InProgress)),
            List(Some(Done)) => self.list(Some(Done)),
        }
    
        Some(())
    }
}