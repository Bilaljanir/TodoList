pub struct Todo {
    pub description: String,
    pub archived: bool,
}

impl Todo {
    pub fn new(description: String) -> Self {
        Todo {
            description,
            archived: false,
        }
    }

    pub fn new_with_status(description: String, archived: bool) -> Self {
        Todo {
            description,
            archived,
        }
    }
}
pub fn display_last_5(tasks: &Vec<Todo>) {

    // Display the last 5 tasks
    for (index, task) in tasks.iter().rev().take(5).enumerate() {
        println!(
            "{}. [{}] {}",
            index + 1,
            if task.archived { "X" } else { " " },
            task.description
        );
    }
}

pub fn display_archived_tasks(tasks: &Vec<Todo>) {
    println!("Tâches archivées :");
    for (index, task) in tasks.iter().enumerate() {
        if task.archived {
            println!("{}. [X] {}", index + 1, task.description);
        }
    }
}


