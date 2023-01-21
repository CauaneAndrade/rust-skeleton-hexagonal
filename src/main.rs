use std::env;
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
struct AppArguments {
    #[clap(short, long = "project-name")]
    project_name: String,

    #[clap(short, long = "project-details")]
    details: String,

    #[clap(short, long)]
    output: Option<String>,
}

impl AppArguments {
    pub fn details(&self) -> &str { &self.details }

    pub fn output(&self) -> Option<String> { self.output.clone() }

    pub fn project_name(&self) -> &str { &self.project_name }
}

struct FileManager {
    base_path: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl FileManager {
    pub fn init(base_path: &Path) -> Result<Self> {
        let base_path = base_path.to_owned();
        if base_path.exists() {
            return Err("folder already exists. try into another folder.".into());
        }

        fs::create_dir_all(&base_path)?;
        Ok(Self {
            base_path,
        })
    }

    pub fn folder_exists(&self, target: &str) -> bool {
        fs::metadata(target).is_ok()
    }

    pub fn create(&self, target: &Path) -> Result<()> {
        if !self.base_path.exists() {
            fs::create_dir_all(&self.base_path)?;
        }

        let target_path = self.base_path.clone().join(target);
        if FileManager::is_file_path(&target_path) {
            FileManager::create_file(&target_path)?;
            return Ok(());
        }

        FileManager::create_folder(&target_path)?;
        Ok(())
    }

    pub fn create_file(path: &Path) -> Result<()> {
        if path.exists() {
            return Err(format!("cannot create file {:?}, file already exists.", path).into())
        }

        if !FileManager::is_file_path(&path) {
            return Err(format!("path is not a file. {:?}", path).into());
        }

        let parent_path = match path.parent() {
            Some(path) => path,
            None => {
                return Err(format!("failed when trying to get parent of {:?}", path).into())
            }
        };

        if !parent_path.exists() {
            FileManager::create_folder(&parent_path)?;
        }

        fs::File::create(&path)?;
        Ok(())
    }

    pub fn create_folder(path: &Path) -> Result<()> {
        if path.exists() {
            return Ok(());
        }

        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn is_file_path(path: &Path) -> bool {
        path.extension().is_some()
    }

    pub fn get_current_path() -> Result<PathBuf> {
        Ok(env::current_dir()?)
    }

    pub fn get_base_path(args: &AppArguments) -> Result<PathBuf> {
        let mut base_path = match args.output() {
            Some(path) => Path::new(&path).to_path_buf(),
            None => env::current_dir()?
        };

        Ok(base_path.join(args.project_name()))
    }

    pub fn write_to_file(&self, target: &Path, buff: &[u8]) -> Result<()> {
        let target = self.base_path.join(&target);
        if !target.exists() {
            return Err(format!("file {:?} doesnt exist.", &target).into());
        }

        let mut file = OpenOptions::new()
            .append(false)
            .write(true)
            .open(&target)?;

        file.write(&buff[..])?;
        file.flush()?;

        Ok(())
    }
}

struct App {
    args: AppArguments,
}


const MAIN_FILE: &str = "src/main.rs";
const CARGO_FILE: &str = "Cargo.toml";

lazy_static! {
    static ref FILES_TO_BE_CREATED: Vec<String> = vec![
        "src/application/mod.rs".to_string(),
        "src/domain/mod.rs".to_string(),
        "src/infrastructure/mod.rs".to_string(),
        MAIN_FILE.to_string(),
        CARGO_FILE.to_string(),
    ];
}

macro_rules! cargo_template_str {
    () => {
r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Your Name"]
description = "{}"#
    }
}

macro_rules! main_template_str {
    () => {
r#"fn main() {
    println!("hello world");
}"#
    }
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
        for file_name in FILES_TO_BE_CREATED.iter() {
            file_manager.create(Path::new(&file_name))?;
        }

        let cargo_file_contents = format!(
            cargo_template_str!(),
            self.args.project_name(),
            self.args.details());

        file_manager.write_to_file(
            Path::new(MAIN_FILE),
            main_template_str!().as_bytes())?;

        file_manager.write_to_file(
            Path::new(CARGO_FILE),
            cargo_file_contents.as_bytes())?;

        Ok(base_path)
    }
}

fn main() -> Result<()> {
    let args = AppArguments::parse();
    let app = App::new(&args);
    match app.run() {
        Ok(created_path) => {
            let msg = format!(
                "Project {} successfully created at {:?}",
                args.project_name(),
                created_path);

            println!("{}", msg)
        },
        Err(err) => println!("failed to create project {}: {}", args.project_name(), err),
    }

    Ok(())
}
