mod task;

use std::fs;
use std::io;
use task::Task;

fn save_tasks(tasks: &Vec<Task>, file_path: &str) {
    let serialized = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(file_path, serialized).expect("Failed to write to file");
}

fn load_tasks(file_path: &str) -> Vec<Task> {
    let data = fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).expect("Failed to deserialize tasks")
}

fn main() {
    let file_path = "tasks.json";
    let mut tasks = load_tasks(file_path);

    loop {
        println!("\nTask Manager");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                let id = (tasks.len() + 1) as u32;
                tasks.push(Task::new(id, description.trim().to_string()));
                save_tasks(&tasks, file_path);
                println!("Task added.");
            }
            "2" => {
                println!("\nTasks:");
                for task in &tasks {
                    println!(
                        "{}. {} [{}]",
                        task.id,
                        task.description,
                        if task.completed { "Done" } else { "Pending" }
                    );
                }
            }
            "3" => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}
