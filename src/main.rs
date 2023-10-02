use std::path::Path;

mod todo;
mod input;
mod file;

fn main() {
    // Check if the backup file exists

    let file_path = "data/tasks.txt";
    let mut tasks = if Path::new(file_path).exists() {
        match file::load_tasks(file_path) {
            Ok(tasks) => tasks,
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    loop {
        println!("--- Todolist ---");
        todo::display_last_5(&tasks);

        println!("Options:");
        println!(" [N] Nouvelle tâche");
        println!(" [A] Afficher les tâches archivées");
        println!(" [Q] Quitter");
        println!(" [R] Archiver une tâche par numéro");


        let choice = input::read_input();

        match choice.trim().to_uppercase().as_str() {
            "N" => {
                let new_task = input::read_input();
                tasks.push(todo::Todo::new(new_task));

                if let Err(err) = file::save_tasks(&tasks, file_path) {
                    eprintln!("Erreur lors de la sauvegarde des tâches : {}", err);
                }
            }
            "R" => {
                println!("Entrez le numéro de la tâche que vous souhaitez archiver : ");
                let task_number = input::read_input().trim().parse::<usize>();

                match task_number {
                    Ok(num) if num > 0 && num <= tasks.len() => {
                        tasks[num -1].archived = true;
                        println!("Tâche {} archivée.", num);

                        if let Err(err) = file::save_tasks(&tasks, file_path) {
                            eprintln!("Erreur lors de la sauvegarde des tâches : {}", err);
                        }
                    }
                    _ => println!("Numéro de tâche invalide."),
                }
            }
            "A" => {
                todo::display_archived_tasks(&tasks);
            }

            "Q" => break,
            _ => println!("Option invalide."),
        }
    }
}
