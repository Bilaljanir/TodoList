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

// load the task list from a file specified by file_path.
pub fn load_tasks(file_path: &str) -> Result<Vec<Todo>> {
    let mut tasks = Vec::new();

    if Path::new(file_path).exists() {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 2 {
                let archived = parts[0].parse::<bool>().unwrap_or(false);
                let description = parts[1].to_string();
                tasks.push(Todo::new_with_status(description, archived));
            }
        }
    }

    Ok(tasks)
}
