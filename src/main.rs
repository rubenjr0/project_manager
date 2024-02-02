use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

mod project;

#[derive(Debug, Parser)]
struct Args {
    path: Option<PathBuf>,
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    New { name: String },
}

fn main() {
    // get an optional path and an optional action
    let args = Args::parse();
    let path = args.path.unwrap_or(std::env::current_dir().unwrap());

    match args.commands {
        Some(Commands::New { name }) => {
            let project = project::Project::new(&name);
            std::fs::write(
                path.join("project.toml"),
                toml::to_string(&project).unwrap(),
            )
            .unwrap();
        }
        None => {
            if is_project(&path) {
                let project = read_project(&path);
                let project = toml::to_string(&project).unwrap();
                println!("{}", project);
            } else {
                list_projects(path);
            }
        }
    }
}

fn is_project(path: &Path) -> bool {
    if path.is_dir() {
        std::fs::read_dir(path).unwrap().any(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            path.is_file() && path.file_name().unwrap() == "project.toml"
        })
    } else {
        false
    }
}

fn find_projects(path: &Path, nested: bool) -> Vec<String> {
    let mut projects = Vec::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if is_project(&path) {
                projects.push(path.canonicalize().unwrap().display().to_string());
            }
            if !nested {
                projects.append(&mut find_projects(&path, true));
            }
        }
    }
    projects
}

fn list_projects(path: PathBuf) {
    let projects = find_projects(&path, false);

    for project in projects {
        let project = read_project(Path::new(&project));
        println!("{}", project.name());
    }
}

fn read_project(path: &Path) -> project::Project {
    let toml = std::fs::read_to_string(path.join("project.toml")).unwrap();
    toml::from_str(&toml).unwrap()
}
