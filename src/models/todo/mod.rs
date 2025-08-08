use deeb::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::CliResult;

#[derive(Serialize, Deserialize, Collection)]
pub struct Todo {
    name: String,
    task: String,
}

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Failed to insert todo.")]
    InsertFailed,
    #[error("Failed to find todos...")]
    FindManyFailed,
}

impl Todo {
    pub async fn new(db: &Deeb, name: String, task: String) -> CliResult<Self> {
        let todo = Todo::insert_one(db, Todo { name, task }, None)
            .await
            .map_err(|_e| TodoError::InsertFailed)?;

        Ok(todo)
    }

    pub async fn list(db: &Deeb) -> CliResult<()> {
        let todos = Todo::find_many(db, Query::All, None, None)
            .await
            .map_err(|_e| TodoError::FindManyFailed)?;

        if todos.is_none() {
            return Ok(());
        }

        for todo in &todos.unwrap() {
            println!("• {} — {}", todo.name, todo.task);
        }


        Ok(())
    }
}
