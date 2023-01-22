use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::args::AppArguments;
use crate::result::Result;

pub struct FileManager {
    base_path: PathBuf,
}

pub enum DiskEntryType {
    Folder,
    File,
}

pub struct DiskEntry {
    base_path: String,
    name: String,
    representation_type: DiskEntryType
}

impl DiskEntry {
    pub fn new(
        base_path: &str,
        name: &str,
        representation_type: DiskEntryType) -> Self {
        Self {
            base_path: String::from(base_path),
            name: String::from(name),
            representation_type,
        }
    }

    pub fn get_full_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.name)
    }
}

impl FileManager {
    pub fn init(base_path: &Path) -> Result<Self> {
        // Convert the input base path to an owned object
        let base_path = base_path.to_owned();
        if base_path.exists() {
            return Err("Error: Folder already exists. Please choose a different location.".into());
        }
    
        // Create the specified directory and all necessary parent directories
        fs::create_dir_all(&base_path)?;
    
        Ok(Self { base_path })
    }

    pub fn folder_exists(&self, target: &str) -> bool {
        fs::metadata(target).is_ok()
    }

    pub fn create(&self, file_representation: &DiskEntry) -> Result<()> {
        if !self.base_path.exists() {
            fs::create_dir_all(&self.base_path)?;
        }

        let file_path = file_representation.get_full_path();
        let target_path = self.base_path
            .clone()
            .join(&file_path);

        match file_representation.representation_type {
            DiskEntryType::File => FileManager::create_file(&target_path)?,
            DiskEntryType::Folder => FileManager::create_folder(&target_path)?,
        };

        Ok(())
    }

    pub fn create_file(path: &Path) -> Result<()> {
        if path.exists() {
            return Err(format!("File {:?} already exists, cannot create duplicate file.", path).into())
        }
        
        // Get the parent directory of the file
        let parent_path = match path.parent() {
            Some(path) => path,
            None => {
                return Err(format!("Unable to determine parent directory of {:?} for file creation.", path).into())
            }
        };

        // If the parent directory doesn't exist, create it
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

    pub fn has_extension(path: &Path) -> bool {
        path.extension().is_some()
    }

    pub fn get_current_path() -> Result<PathBuf> {
        Ok(env::current_dir()?)
    }

    pub fn get_base_path(args: &AppArguments) -> Result<PathBuf> {
        let base_path = match args.output() {
            Some(path) => Path::new(&path).to_path_buf(),
            None => env::current_dir()?
        };

        Ok(base_path.join(args.project_name()))
    }

    pub fn write_to_file(&self, target: &Path, buff: &[u8]) -> Result<()> {
        let target = self.base_path.join(&target);
        if !target.exists() {
            return Err(format!("File {:?} does not exist.", &target).into());
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

