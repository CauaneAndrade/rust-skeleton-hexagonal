use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct AppArguments {
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