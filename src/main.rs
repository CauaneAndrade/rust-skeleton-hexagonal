use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Ask for project name
    println!("Please enter a name for your project:");
    let mut project_name = String::new();
    std::io::stdin().read_line(&mut project_name).unwrap();
    let project_name = project_name.trim();

    // Ask for project details
    println!("Please enter a brief description of your project:");
    let mut project_description = String::new();
    std::io::stdin().read_line(&mut project_description).unwrap();
    let project_description = project_description.trim();

    println!("Please enter the path where you want to create the project (leave blank for current directory):");
    let mut project_path = String::new();
    std::io::stdin().read_line(&mut project_path).unwrap();
    let project_path = project_path.trim();

    let project_path = if project_path.is_empty() {
        env::current_dir().unwrap()
    } else {
        Path::new(&project_path).to_path_buf()
    };

    // Create the project directory
    fs::create_dir_all(project_path.join(project_name)).unwrap();

    // Create the hexagonal architecture directories and files
    fs::create_dir_all(project_path.join(project_name).join("src")).unwrap();
    fs::create_dir_all(project_path.join(project_name).join("src").join("application")).unwrap();
    fs::create_dir_all(project_path.join(project_name).join("src").join("domain")).unwrap();
    fs::create_dir_all(project_path.join(project_name).join("src").join("infrastructure")).unwrap();

    // Create main.rs file
    let main_file = project_path.join(project_name).join("src").join("main.rs");
    fs::File::create(main_file).unwrap();

    println!("Successfully created new project at {:?} with name {} and description: {}", project_path, project_name, project_description);
}
