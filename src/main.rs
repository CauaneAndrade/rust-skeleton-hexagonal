use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // Prompt user for project name
    let mut project_name = String::new();
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut project_name).unwrap();
    let project_name = project_name.trim();
    // Ask for project details
    println!("Please enter a brief description of your project:");
    let mut project_description = String::new();
    std::io::stdin()
        .read_line(&mut project_description)
        .unwrap();
    let project_description = project_description.trim();
    // Prompt user for location to create project
    print!("Enter location to create project: ");
    io::stdout().flush().unwrap();
    let mut project_location = String::new();
    io::stdin().read_line(&mut project_location).unwrap();
    let project_location = project_location.trim();
    // If user did not provide location, create project in current directory
    let current_dir = env::current_dir().unwrap();
    let project_path = if project_location.is_empty() {
        current_dir
    } else {
        Path::new(&project_location).to_path_buf()
    };

    // Create the project directory
    fs::create_dir_all(project_path.join(project_name)).unwrap();
    // Create the hexagonal architecture directories and files
    fs::create_dir_all(project_path.join(project_name).join("src")).unwrap();
    fs::create_dir_all(
        project_path
            .join(project_name)
            .join("src")
            .join("application"),
    )
    .unwrap();
    fs::create_dir_all(project_path.join(project_name).join("src").join("domain")).unwrap();
    fs::create_dir_all(
        project_path
            .join(project_name)
            .join("src")
            .join("infrastructure"),
    )
    .unwrap();
    // Create files in the application folder
    let application_file = project_path
        .join(project_name)
        .join("src")
        .join("application")
        .join("mod.rs");
    fs::File::create(application_file).unwrap();

    // Create files in the domain folder
    let domain_file = project_path
        .join(project_name)
        .join("src")
        .join("domain")
        .join("mod.rs");
    fs::File::create(domain_file).unwrap();

    // Create files in the infrastructure folder
    let infrastructure_file = project_path
        .join(project_name)
        .join("src")
        .join("infrastructure")
        .join("mod.rs");
    fs::File::create(infrastructure_file).unwrap();

    // Create files in the application folder
    let application_file = project_path
        .join(project_name)
        .join("src")
        .join("application")
        .join("mod.rs");
    fs::File::create(application_file).unwrap();

    // Create main.rs file
    let main_file = project_path.join(project_name).join("src").join("main.rs");
    fs::File::create(main_file).unwrap();

    // Create cargo.toml file
    let cargo_toml_path = project_path.join(project_name).join("Cargo.toml");
    fs::File::create(&cargo_toml_path).unwrap();

    // Write to cargo.toml file
    let mut cargo_toml = fs::OpenOptions::new()
        .write(true)
        .open(&cargo_toml_path)
        .unwrap();
    cargo_toml
        .write_all(
            format!(
                r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Your Name"]
description = "{}"

[dependencies]
"#,
                project_name, project_description
            )
            .as_bytes(),
        )
        .unwrap();

    println!(
        "Project {} successfully created at {:?}",
        project_name, project_path
    );
}
