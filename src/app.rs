use crate::args::AppArguments;
use crate::file_manager::{DiskEntry, DiskEntryType, FileManager};
use lazy_static::lazy_static;
use std::path::PathBuf;

use crate::result::Result;

const CARGO_FILE_PATH: &str = "";
const CARGO_FILE_NAME: &str = "Cargo.toml";
const MAIN_FILE_PATH: &str = "src";
const MAIN_FILE_NAME: &str = "main.rs";

// Initialize code in a static way with lazy_static! macro for performance optimization
lazy_static! {
    static ref FILES_TO_BE_CREATED: Vec<DiskEntry> = vec![
        DiskEntry::new("src/application", "mod.rs", DiskEntryType::File),
        DiskEntry::new("src/domain", "mod.rs", DiskEntryType::File),
        DiskEntry::new("src/infrastructure", "mod.rs", DiskEntryType::File),
        DiskEntry::new(CARGO_FILE_PATH, CARGO_FILE_NAME, DiskEntryType::File),
        DiskEntry::new(MAIN_FILE_PATH, MAIN_FILE_NAME, DiskEntryType::File),
    ];
}

/// A macro function for generating the main template string for a Rust program.
macro_rules! main_template_str {
    () => {
        r#"fn main() {
    println!("Hello, world!");
}"#
    };
}

/// Using a macro allows for easy modification and maintenance of the Cargo.toml template string.
macro_rules! cargo_template_str {
    () => {
        r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Your Name"]
description = "{}"#
    };
}

// App struct is used to hold the arguments passed to the program.
pub struct App {
    args: AppArguments,
}

impl App {
    pub fn new(args: &AppArguments) -> Self {
        Self { args: args.clone() }
    }

    // Returns a Result type containing the base path of the project.
    pub fn run(&self) -> Result<PathBuf> {
        let base_path = FileManager::get_base_path(&self.args)?;
        let file_manager = FileManager::init(&base_path)?;
        for file_representation in FILES_TO_BE_CREATED.iter() {
            file_manager.create(&file_representation)?;
        }

        let cargo_file_contents = format!(
            cargo_template_str!(),
            self.args.project_name(),
            self.args.details()
        );

        let main_file = DiskEntry::new(MAIN_FILE_PATH, MAIN_FILE_NAME, DiskEntryType::File);

        let cargo_file = DiskEntry::new(CARGO_FILE_PATH, CARGO_FILE_NAME, DiskEntryType::File);

        file_manager.write_to_file(&main_file.get_full_path(), main_template_str!().as_bytes())?;

        file_manager.write_to_file(&cargo_file.get_full_path(), cargo_file_contents.as_bytes())?;

        Ok(base_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::AppArguments;
    use std::fs;
    use tempfile::tempdir;

    fn setup() -> (App, PathBuf) {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        let args = AppArguments::new(
            "test_project",
            "test details",
            Some(temp_path.to_str().unwrap().to_owned()),
        );
        let app = App::new(&args);
        (app, temp_path)
    }

    #[test]
    fn test_app_run_success() {
        let (app, temp_path) = setup();

        let result = app.run();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_path.join("test_project"));
    }

    #[test]
    fn test_app_run_fails_folder_exists() {
        let (app, _) = setup();

        let result = app.run();
        let result2 = app.run();

        assert!(result.is_ok());
        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err().to_string(),
            format!("Error: Folder already exists. Please choose a different location.")
        );
    }

    #[test]
    fn test_file_contents() {
        let (app, temp_path) = setup();

        app.run().unwrap();

        let cargo_file = temp_path.join("test_project").join(CARGO_FILE_NAME);
        let main_file = temp_path
            .join("test_project")
            .join(MAIN_FILE_PATH)
            .join(MAIN_FILE_NAME);

        let cargo_file_contents = fs::read_to_string(cargo_file).unwrap();
        let main_file_contents = fs::read_to_string(main_file).unwrap();

        assert_eq!(
            cargo_file_contents,
            format!(cargo_template_str!(), "test_project", "test details")
        );
        assert_eq!(main_file_contents, main_template_str!());
    }
}
