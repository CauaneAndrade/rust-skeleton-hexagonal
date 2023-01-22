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
                "The project '{}' was successfully created at {:?}.",
                args.project_name(),
                created_path));
        },
        Err(err) =>{
            println!(
                "Failed to create project '{}': {}",
                args.project_name(),
                err)
        },
    }

    Ok(())
}
