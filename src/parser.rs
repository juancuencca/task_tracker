use crate::{Result, Status};

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