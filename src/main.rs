use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}
enum Command {
    Add,
    List,
    Done,
    Undone,
    Delete,
    Menu,
    Exit,
    Empty,
}
fn parse_command(input: &str) -> Command {
    match input {
        "add" => Command::Add,
        "list" => Command::List,
        "done" => Command::Done,
        "undone" => Command::Undone,
        "delete" => Command::Delete,
        "menu" => Command::Menu,
        "exit" => Command::Exit,
        &_ => Command::Empty,
    }
}
//function to get file path
fn get_tasks_file_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "yourname", "todo_list") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir).ok()?; // create dir if needed
        Some(data_dir.join("tasks.json"))
    } else {
        None
    }
}
//Function to print the menu
fn print_menu() {
    println!("=======================");
    println!("      TODO APP MENU    ");
    println!("=======================");
    println!("add     - Add a task");
    println!("list    - List all tasks");
    println!("done    - Mark a task as done");
    println!("undone    - Mark a task as undone");
    println!("delete  - Remove a task");
    println!("menu    - Print the menu");
    println!("exit    - Quit the app");
    println!("=======================\n");
    println!("Enter your choice: \n");
}

//Funciton to add task
fn add_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    let _file_path = get_tasks_file_path().expect("Could not determine tasks file path");

    println!("Enter description of your task \n");

    let mut description = String::new();

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");
    description = description.trim().to_string();

    let task = Task {
        description,
        done: false,
    };
    tasks.push(task);

    // // Serialize the full updated list
    let serialized = serde_json::to_string_pretty(&tasks).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
    })?;

    // Save to file
    let mut file = File::create(_file_path)?;
    file.write_all(serialized.as_bytes())?;

    match list_task(&tasks) {
        Ok(()) => println!(""),
        Err(e) => eprintln!("Failed to add task: {}", e),
    }

    Ok(())
}

fn get_tasks() -> io::Result<Vec<Task>> {
    let file_path = get_tasks_file_path().expect("Could not determine tasks file path");

    // Try to open the file
    if let Ok(mut file) = File::open(file_path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // If file is empty, no tasks to show
        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }

        let tasks: Vec<Task> = serde_json::from_str(&contents).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Deserialization failed: {}", e),
            )
        })?;

        Ok(tasks)
    } else {
        // File doesn't exist or couldn't be opened
        println!("No tasks file found.");
        Ok(Vec::new())
    }
}
fn list_task(tasks: &Vec<Task>) -> io::Result<()> {
    if tasks.is_empty() {
        println!("\nNo tasks available.");
    } else {
        println!("\nYour Tasks:");
        println!("-----------");

        // Print each task with index and status
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done { "[✓]" } else { "[✖]" };
            println!("{} {} - {}", i + 1, status, task.description);
        }
    }

    Ok(())
}

//funciton that mars a task done
fn done_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    match list_task(&tasks) {
        Ok(()) => println!(""),
        Err(e) => eprintln!("Failed to add task: {}", e),
    }

    let _file_path = get_tasks_file_path().expect("Could not determine tasks file path");
    loop {
        println!("\nSelect by index to mark task as done: (ie: 1, 2 )");

        let mut index_input = String::new();

        io::stdin()
            .read_line(&mut index_input)
            .expect("Couldnot take input");

        let index: i32 = match index_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid index please input a number");
                continue;
            }
        };
        println!("You chose {}", index);

        if index > 0 {
            //holy fucking fuck wtf usize for indexing??????
            //it checks if index is greateer than 0 and smaller or equal to length of tasks vector
            //because i cannot do -1 indexing and also cant index tasks[4] if the taskslength is
            //only 3
            //
            if index > 0 && (index as usize) <= tasks.len() {
                if tasks[(index - 1) as usize].done {
                    println!("Task is already marked as done.");
                } else {
                    tasks[(index - 1) as usize].done = true;

                    match list_task(&tasks) {
                        Ok(()) => println!(""),
                        Err(e) => eprintln!("Failed to add task: {}", e),
                    }
                }
            } else {
                println!("Invalid index!");
            }

            let serialized = serde_json::to_string_pretty(&tasks).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
            })?;

            // Save to file
            let mut file = File::create(_file_path)?;
            file.write_all(serialized.as_bytes())?;
            break;
        }
    }
    // tasks.extend(existing_tasks);
    Ok(())
}

//funciton that mars a task undone
fn undone_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    match list_task(&tasks) {
        Ok(()) => println!(""),
        Err(e) => eprintln!("Failed to add task: {}", e),
    }

    let _file_path = get_tasks_file_path().expect("Could not determine tasks file path");
    loop {
        println!("\nSelect by index to mark task as undone: (ie: 1, 2 )");

        let mut index_input = String::new();

        io::stdin()
            .read_line(&mut index_input)
            .expect("Couldnot take input");

        let index: i32 = match index_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid index please input a number");
                continue;
            }
        };
        println!("You chose {}", index);

        if index > 0 {
            //holy fucking fuck wtf usize for indexing??????
            //it checks if index is greateer than 0 and smaller or equal to length of tasks vector
            //because i cannot do -1 indexing and also cant index tasks[4] if the taskslength is
            //only 3
            //
            if index > 0 && (index as usize) <= tasks.len() {
                if !tasks[(index - 1) as usize].done {
                    println!("Task is already marked as undone.");
                } else {
                    tasks[(index - 1) as usize].done = false;

                    match list_task(&tasks) {
                        Ok(()) => println!(""),
                        Err(e) => eprintln!("Failed to add task: {}", e),
                    }
                }
            } else {
                println!("Invalid index!");
            }

            let serialized = serde_json::to_string_pretty(&tasks).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
            })?;

            // Save to file
            let mut file = File::create(_file_path)?;
            file.write_all(serialized.as_bytes())?;
            break;
        }
    }
    // tasks.extend(existing_tasks);
    Ok(())
}

//Delete task func definition
fn delete_task(tasks: &mut Vec<Task>) -> io::Result<()> {
    let _file_path = get_tasks_file_path().expect("Could not determine tasks file path");
    if tasks.is_empty() {
        println!("No tasks to delete.");
        return Ok(());
    }
    match list_task(&tasks) {
        Ok(()) => println!(""),
        Err(e) => eprintln!("Failed to add task: {}", e),
    }
    loop {
        println!("\nSelect by index to delete task: (ie: 1, 2 )");

        let mut index_input = String::new();

        io::stdin()
            .read_line(&mut index_input)
            .expect("Couldnot take input");

        let index: i32 = match index_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid index please input a number");
                continue;
            }
        };
        println!("You chose {}", index);
        if index > 0 {
            //holy fucking fuck wtf usize for indexing??????
            //it checks if index is greateer than 0 and smaller or equal to length of tasks vector
            //because i cannot do -1 indexing and also cant index tasks[4] if the taskslength is
            //only 3
            if index > 0 && (index as usize) <= tasks.len() {
                let deleted_task = tasks.remove((index - 1) as usize);

                //display the deleted task
                let status = if deleted_task.done { "✓" } else { "✖" };
                println!(
                    "{}. {} [{}] Was deleted",
                    index, deleted_task.description, status
                );
            } else {
                println!("Invalid index!");
            }

            let serialized = serde_json::to_string_pretty(&tasks).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
            })?;

            // Save to file
            let mut file = File::create(_file_path)?;
            file.write_all(serialized.as_bytes())?;
            break;
        }
    }

    match list_task(&tasks) {
        Ok(()) => println!("\n"),
        Err(e) => eprintln!("Failed to list: {}", e),
    }
    Ok(())
}

fn main() {
    let mut tasks = match get_tasks() {
        Ok(task) => task,
        _ => return,
    };

    match list_task(&tasks) {
        Ok(()) => println!("\n"),
        Err(e) => eprintln!("Failed to list: {}", e),
    }
    print_menu();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input = input.trim().to_lowercase().to_string();
        let command = parse_command(&input);
        match command {
            Command::Add => match add_task(&mut tasks) {
                Ok(()) => println!("Task added successfully!\n"),
                Err(e) => eprintln!("Failed to add task: {}", e),
            },
            Command::List => match list_task(&tasks) {
                Ok(()) => println!("\n"),
                Err(e) => eprintln!("Failed to list: {}", e),
            },
            Command::Done => match done_task(&mut tasks) {
                Ok(()) => println!("\n"),
                Err(e) => eprintln!("Failed to list: {}", e),
            },
            Command::Undone => match undone_task(&mut tasks) {
                Ok(()) => println!("\n"),
                Err(e) => eprintln!("Failed to list: {}", e),
            },
            Command::Delete => match delete_task(&mut tasks) {
                Ok(()) => println!("\n"),
                Err(e) => eprintln!("Failed to list: {}", e),
            },

            Command::Menu => print_menu(),
            Command::Exit => break,
            Command::Empty => {
                println!("No commands found re renter command");
                continue;
            }
        }
    }
}
