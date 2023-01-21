use std::env;
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

use rustskeleton::app::App;
use rustskeleton::args::AppArguments;
use rustskeleton::result::Result;

fn main() -> Result<()> {
    let args = AppArguments::parse();
    let app = App::new(&args);

    // TODO: Write Unit tests for all code.

    // TODO: refactor this to use a Enum error + Display impl approach.
    match app.run() {
        Ok(created_path) => {
            println!("{}", format!(
                "Project {} successfully created at {:?}",
                args.project_name(),
                created_path));
        },
        Err(err) =>{
            println!(
                "failed to create project {}: {}",
                args.project_name(),
                err)
        },
    }

    Ok(())
}
