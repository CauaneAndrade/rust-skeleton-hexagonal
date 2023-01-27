# RustSkeleton
[![License](https://img.shields.io/badge/License-GNUv3.0-blue.svg)](https://opensource.org/licenses/GNU)
[![Rust](https://img.shields.io/badge/Rust-1.63.0-orange.svg)](https://www.rust-lang.org/)

RustSkeleton is a command-line tool that helps developers quickly set up a new Rust project using the hexagonal architecture pattern. With RustSkeleton, you can easily create a well-structured and maintainable project, without having to manually set up the directory structure and boilerplate code.

## Features

- Asks for project name, description, and location to create the project, allowing for customization
- Creates a cargo.toml file with basic information about the project
- Creates main.rs file in the appropriate folder
- Creates application, domain, and infrastructure folders with appropriate files.
- It organizes the code in a way that makes it easy to separate concerns and reason about the different parts of the code.

## Usage
```bash
rustskeleton -h
```

```bash
rustskeleton --project-name <PROJECT_NAME> --project-details <DETAILS>
```

### How to use
- When prompted, enter a name for your project
- Then enter a brief description of your project
- Then enter the location to create the project, or leave it blank to create in the current directory
- The tool will then create the file and directory structure for your new Rust project using the hexagonal architecture pattern

## Contributing
Pull requests are welcome. Please make sure to update tests as appropriate.