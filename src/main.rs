use std::borrow::Cow;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct User {
    #[allow(dead_code)]
    id: usize,
    name: String,
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    owner: usize, // User ID of the owner
}

struct TaskManager {
    users: HashMap<usize, User>,
    tasks: HashMap<usize, Task>,
    next_task_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            tasks: HashMap::new(),
            next_task_id: 1,
        }
    }

    fn add_user(&mut self, id: usize, name: String) {
        self.users.insert(id, User { id, name });
    }

    fn add_task(&mut self, owner_id: usize, description: String) -> Result<(), String> {
        if !self.users.contains_key(&owner_id) {
            return Err("User does not exist".to_string());
        }
        let task = Task {
            id: self.next_task_id,
            description,
            completed: false,
            owner: owner_id,
        };
        self.tasks.insert(self.next_task_id, task);
        self.next_task_id += 1;
        Ok(())
    }

    fn transfer_task(&mut self, task_id: usize, new_owner_id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            if self.users.contains_key(&new_owner_id) {
                // Transfer ownership
                let old_owner_id = task.owner;
                task.owner = new_owner_id;
                println!(
                    "Task {} transferred from User {} to User {}.",
                    task.id, old_owner_id, new_owner_id
                );
                Ok(())
            } else {
                Err("New owner does not exist".to_string())
            }
        } else {
            Err("Task not found".to_string())
        }
    }

    fn complete_task(&mut self, task_id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.completed = true;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            let owner_name: Cow<str> = self
                .users
                .get(&task.owner)
                .map(|user| Cow::Borrowed(user.name.as_str()))
                .unwrap_or_else(|| Cow::Owned("Unknown".to_string()));

            println!(
                "Task {}: {} (Owner: {}, Completed: {})",
                task.id, task.description, owner_name, task.completed
            );
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();

    // Adding users interactively
    loop {
        println!("Enter a user ID (or 'done' to stop): ");
        let mut user_id = String::new();
        io::stdin()
            .read_line(&mut user_id)
            .expect("Failed to read line");
        let user_id = user_id.trim();
        if user_id == "done" {
            break;
        }

        let user_id: usize = user_id.parse().expect("Please enter a valid number");

        println!("Enter the user's name: ");
        let mut user_name = String::new();
        io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read line");
        let user_name = user_name.trim().to_string();

        manager.add_user(user_id, user_name);
    }

    // Adding tasks interactively
    loop {
        println!("\nEnter a task description (or 'done' to stop): ");
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");
        let description = description.trim();
        if description == "done" {
            break;
        }

        println!("Enter owner ID: ");
        let mut owner_id = String::new();
        io::stdin()
            .read_line(&mut owner_id)
            .expect("Failed to read line");
        let owner_id: usize = owner_id
            .trim()
            .parse()
            .expect("Please enter a valid number");

        if let Err(e) = manager.add_task(owner_id, description.to_string()) {
            println!("Error: {}", e);
        }
    }

    // List tasks
    println!("\nCurrent tasks:");
    manager.list_tasks();

    // Transfer a task
    println!("\nEnter task ID to transfer: ");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");
    let task_id: usize = task_id.trim().parse().expect("Please enter a valid number");

    println!("Enter new owner ID: ");
    let mut new_owner_id = String::new();
    io::stdin()
        .read_line(&mut new_owner_id)
        .expect("Failed to read line");
    let new_owner_id: usize = new_owner_id
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if let Err(e) = manager.transfer_task(task_id, new_owner_id) {
        println!("Error: {}", e);
    }

    // List tasks after transfer
    println!("\nTasks after transfer:");
    manager.list_tasks();

    // Mark a task as completed
    println!("\nEnter task ID to complete: ");
    let mut complete_task_id = String::new();
    io::stdin()
        .read_line(&mut complete_task_id)
        .expect("Failed to read line");
    let complete_task_id: usize = complete_task_id
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if let Err(e) = manager.complete_task(complete_task_id) {
        println!("Error: {}", e);
    }

    // List tasks after completion
    println!("\nTasks after completion:");
    manager.list_tasks();
}
