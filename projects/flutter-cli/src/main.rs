use dialoguer::{theme::ColorfulTheme, Input, Select};
use native_dialog::FileDialog;
use std::{env, fs, process::Command};

fn create_flutter_project(project_name: &str) {
    println!("Creating Flutter project: {}", project_name);

    let status = Command::new("flutter")
        .arg("create")
        .arg("--template")
        .arg("app")
        .arg("--empty")
        .arg("--overwrite")
        .arg("--project-name")
        .arg(project_name)
        .arg(".")
        .status()
        .expect("Failed to execute Flutter command");

    if status.success() {
        println!("Flutter project '{}' created successfully!", project_name);

        // Step 2: Initialize Git repository
        let git_status = Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to execute Git command");

        if git_status.success() {
        } else {
            eprintln!("Failed to initialize Git repository.");
        }
    } else {
        eprintln!("Failed to create Flutter project.");
    }
}

fn main() {
    let items = vec!["Create a flutter project", "cancel"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you choose?")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    match selection {
        0 => {
            println!("Please select the output directory:");

            let output_dir = FileDialog::new()
                .show_open_single_dir()
                .expect("Failed to open file dialog")
                .expect("No directory selected");

            println!("Selected output directory: {}", output_dir.display());

            // Step 2: Project name prompt
            let project_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the project name")
                .interact_text()
                .unwrap();

            let project_path = output_dir.join(&project_name);
            if let Err(e) = fs::create_dir_all(&project_path) {
                eprintln!("Failed to create project folder: {}", e);
                return;
            }

            println!("Created project folder: {}", project_path.display());

            // Step 4: Change to the project directory
            if let Err(e) = env::set_current_dir(&project_path) {
                eprintln!("Failed to change directory to project folder: {}", e);
                return;
            }
            create_flutter_project(&project_name);
        }
        1 => {
            println!("Cancelled");
        }
        _ => println!("please select valid option"),
    }
}
