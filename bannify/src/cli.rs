//#// cli.rs - Command parser

use crate::file_handler::{format_dir, format_file};
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "Bannify", version, about = "being CLI tool for creating banners in code", long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    path: Option<String>,

    #[arg(short, long)]
    exclude: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Format {
        file: String,
    },
    FormatDir {
        dir: String,

        #[arg(short, long)]
        exclude: Vec<String>,
    },
    AddLang {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        single_line_comment: String,

        #[arg(short, long)]
        extensions: Vec<String>,
    },
    ListLang,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Some(Commands::Format { file }) => {
                format_file(file).unwrap();
            }
            Some(Commands::FormatDir { dir, exclude }) => {
                format_dir(dir, exclude).unwrap();
            }
            Some(Commands::AddLang {
                name,
                single_line_comment,
                extensions,
            }) => {
                crate::language::add_language(name, single_line_comment, extensions.clone())
                    .unwrap();
            }
            Some(Commands::ListLang) => {
                crate::language::list_languages().unwrap();
            }
            None => {
                if let Some(path) = &self.path {
                    let path = Path::new(path);
                    if path.is_file() {
                        format_file(path.to_str().unwrap()).unwrap();
                    } else if path.is_dir() {
                        format_dir(path.to_str().unwrap(), &self.exclude).unwrap();
                    } else {
                        eprintln!("Invalid path: {}", path.display());
                    }
                } else {
                    eprintln!("No command provided. Use `--help` for available commands.");
                }
            }
        }
    }
}
