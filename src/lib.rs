use chrono::prelude::*;
use std::fmt;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub enum Action {
    Add(String),
    Update { id: usize, task: String },
    Delete(usize),
    MarkInProgress(usize),
    MarkDone(usize),
    List(Option<Status>),
}

impl Action {
    pub fn build<I: Iterator<Item=String>>(mut args: I) -> Result<Action> {
        args.next();

        let action = match args.next() {
            Some(action) => action,
            None => return Err("action was not provided".into())
        };
    
        match action.as_str() {
            "add" => {
                match args.next() {
                    Some(task) => return Ok(Action::Add(task)),
                    None => return Err("task description was not provided to add task".into()),
                }
            },
            "update" => {
                let id = match args.next() {
                    Some(task_id) => match task_id.parse::<usize>() {
                        Ok(id) => id,
                        Err(e) => return Err(format!("{}", e).into())
                    }, 
                    None => return Err("id was not provided to update task".into()),
                };
    
                match args.next() {
                    Some(task) => return Ok(Action::Update{id, task}),
                    None => return Err("task description was not provided to update task".into()), 
                }
            },
            "delete" => {
                match args.next() {
                    Some(task_id) => match task_id.parse::<usize>() {
                        Ok(id) => return Ok(Action::Delete(id)),
                        Err(e) => return Err(format!("{}", e).into())
                    }, 
                    None => return Err("id was not provided to delete task".into()),
                };
            },
            "mark-in-progress" => {
                match args.next() {
                    Some(task_id) => match task_id.parse::<usize>() {
                        Ok(id) => return Ok(Action::MarkInProgress(id)),
                        Err(e) => return Err(format!("{}", e).into())
                    }, 
                    None => return Err("id was not provided to mark task in progress".into()),
                };
            },
            "mark-done" => {
                match args.next() {
                    Some(task_id) => match task_id.parse::<usize>() {
                        Ok(id) => return Ok(Action::MarkDone(id)),
                        Err(e) => return Err(format!("{}", e).into())
                    }, 
                    None => return Err("id was not provided to mark task done".into()),
                };
            },
            "list" => {
                match args.next() {
                    Some(status) => {
                        match status.as_str() {
                            "todo" => Ok(Action::List(Some(Status::Todo))),
                            "in-progress" => Ok(Action::List(Some(Status::InProgress))),
                            "done" => Ok(Action::List(Some(Status::Done))),
                            unknown => return Err(format!("status {} is not contemplated", unknown).into())  
                        }
                    },
                    None => return Ok(Action::List(None)),
                }
            },
            unknown => return Err(format!("action {} not contemplated", unknown).into()),
        }
    }
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
        write!(f, "Task {{")?;
        write!(f, "  Id: {}", self.id)?;
        write!(f, "  Description: {}", self.description)?;
        write!(f, "  Status: {:?}", self.status)?;
        write!(f, "  Created at: {}", self.created_at)?;
        write!(f, "  Updated at: {:?}", self.updated_at)?;
    
        write!(f, "}}")
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
}