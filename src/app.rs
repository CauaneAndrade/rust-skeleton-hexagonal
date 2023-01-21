use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use crate::args::AppArguments;
use crate::file_manager::{FileManager, DiskEntry, DiskEntryType};

use crate::result::Result;

const CARGO_FILE_PATH: &str = "";
const CARGO_FILE_NAME: &str = "Cargo.toml";
const MAIN_FILE_PATH: &str = "src";
const MAIN_FILE_NAME: &str = "main.rs";

/// This one is cool, you can initialize code in a static way
/// I've created this FileRepresentation struct to better represent a file vs a folder
/// I know that's over-engineering but it allows further extensions on the future.
lazy_static! {
    static ref FILES_TO_BE_CREATED: Vec<DiskEntry> = vec![
        DiskEntry::new("src/application", "mod.rs", DiskEntryType::File),
        DiskEntry::new("src/domain", "mod.rs", DiskEntryType::File),
        DiskEntry::new("src/infrastructure", "mod.rs", DiskEntryType::File),
        DiskEntry::new(CARGO_FILE_PATH, CARGO_FILE_NAME, DiskEntryType::File),
        DiskEntry::new(MAIN_FILE_PATH, MAIN_FILE_NAME, DiskEntryType::File),
    ];
}

/// A tricky way of saving space on your format!(..) function :D
/// There's probably better ways of doing that (like a static file or something)
/// But if that's something that wont change much, i don't see much problem with it.
macro_rules! main_template_str {
    () => {
r#"fn main() {
    println!("hello world");
}"#
    }
}

/// You can even set placeholders so format can replace them :D
macro_rules! cargo_template_str {
    () => {
r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Your Name"]
description = "{}"#
    }
}


pub struct App {
    args: AppArguments,
}

impl App {
    pub fn new(args: &AppArguments) -> Self {
        Self {
            args: args.clone(),
        }
    }

    pub fn run(&self) -> Result<PathBuf> {
        let base_path = FileManager::get_base_path(&self.args)?;
        let file_manager = FileManager::init(&base_path)?;
        for file_representation in FILES_TO_BE_CREATED.iter() {
            file_manager.create(&file_representation)?;
        }

        let cargo_file_contents = format!(
            cargo_template_str!(),
            self.args.project_name(),
            self.args.details());

        let main_file = DiskEntry::new(
            MAIN_FILE_PATH,
            MAIN_FILE_NAME,
            DiskEntryType::File);

        let cargo_file = DiskEntry::new(
                  CARGO_FILE_PATH,
                  CARGO_FILE_NAME,
            DiskEntryType::File);

        file_manager.write_to_file(
            &main_file.get_full_path(),
            main_template_str!().as_bytes())?;

        file_manager.write_to_file(
            &cargo_file.get_full_path(),
            cargo_file_contents.as_bytes())?;

        Ok(base_path)
    }
}
