use std::path::Path;

mod todo;
mod input;
mod file;

fn main() {
    // Vérifiez si le fichier de sauvegarde existe
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
        todo::display_last_5(&tasks); // Afficher les 5 dernières tâches

        println!("Options:");
        println!("  [N] Nouvelle tâche");
        println!("  [A] Afficher les tâches archivées");
        println!("  [Q] Quitter");

        let choice = input::read_input();

        match choice.trim().to_uppercase().as_str() {
            "N" => {
                let new_task = input::read_input();
                tasks.push(todo::Todo::new(new_task));

                if let Err(err) = file::save_tasks(&tasks, file_path) {
                    eprintln!("Erreur lors de la sauvegarde des tâches : {}", err);
                }
            }
            "A" => todo::display_archived(&tasks),
            "Q" => break,
            _ => println!("Option invalide."),
        }
    }
}
