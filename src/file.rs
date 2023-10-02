use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Result};
use std::path::Path;
use crate::todo::Todo;

pub fn save_tasks(tasks: &Vec<Todo>, file_path: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    for task in tasks {
        writeln!(file, "{}|{}", task.archived, task.description)?;
    }

    Ok(())
}
