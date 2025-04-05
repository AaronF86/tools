//------------------------------------------//
//                                          //
// cli.rs - Command parser                  //
//                                          //
//------------------------------------------//


use crate::file_handler::{format_dir, format_file};
use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::exit;

/*
 * Command Structure:
 * 
 * bannify [PATH] [--exclude <patterns>...] [-r|--recursive]     # Process a file or directory
 * bannify format <file>                                         # Format a single file
 * bannify format-dir <dir> [--exclude <patterns>] [-r|--recursive] # Format a directory
 * bannify add-lang --name <name> -s <prefix> -e <exts>...       # Add language
 * bannify list-lang                                             # List supported languages
 */

#[derive(Parser)]
#[command(name = "Bannify", version, about = "CLI tool for creating banners in code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to file or directory to process
    path: Option<String>,

    #[arg(short, long, help = "Patterns to exclude")]
    exclude: Vec<String>,

    #[arg(short, long, help = "Process directories recursively")]
    recursive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Format a single file
    Format {
        /// Path to the file
        file: String,
    },
    
    /// Format all files in a directory
    FormatDir {
        /// Path to the directory
        dir: String,

        #[arg(short, long, help = "Patterns to exclude")]
        exclude: Vec<String>,

        #[arg(short, long, help = "Process directories recursively")]
        recursive: bool,
    },
    
    /// Add a new language to the supported languages
    AddLang {
        #[arg(short, long, help = "Name of the language")]
        name: String,

        #[arg(short, long, help = "Single line comment prefix (e.g. // for C, # for Python)")]
        single_line_comment: String,

        #[arg(short, long, help = "File extensions for this language (e.g. rs, py)")]
        extensions: Vec<String>,
    },
    
    /// List all supported languages
    ListLang,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Some(Commands::Format { file }) => {
                if let Err(e) = format_file(file) {
                    eprintln!("❌ Error formatting file '{}': {}", file, e);
                    exit(1);
                }
                println!("✅ Successfully formatted file: {}", file);
            }
            Some(Commands::FormatDir { dir, exclude, recursive }) => {
                match format_dir(dir, exclude, *recursive) {
                    Ok(count) => {
                        if count > 0 {
                            println!("✅ Successfully formatted {} file{} in directory{}: {}", 
                                count,
                                if count == 1 { "" } else { "s" },
                                if *recursive { " (recursively)" } else { "" }, 
                                dir);
                        } else {
                            println!("ℹ️ No files were formatted in directory{}: {}", 
                                if *recursive { " (recursively)" } else { "" }, 
                                dir);
                        }
                    },
                    Err(e) => {
                        eprintln!("❌ Error formatting directory '{}': {}", dir, e);
                        exit(1);
                    }
                }
            }
            Some(Commands::AddLang {
                name,
                single_line_comment,
                extensions,
            }) => {
                match crate::language::add_language(name, single_line_comment, extensions.clone()) {
                    Ok(_) => {
                        println!("✅ Successfully added language: {} with extensions: {:?}", name, extensions);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to add language '{}': {}", name, e);
                        exit(1);
                    }
                }
            }
            Some(Commands::ListLang) => {
                if let Err(e) = crate::language::list_languages() {
                    eprintln!("❌ Failed to list languages: {}", e);
                    exit(1);
                }
            }
            None => {
                if let Some(path) = &self.path {
                    let path = Path::new(path);
                    if path.is_file() {
                        let path_str = path.to_str().unwrap();
                        if let Err(e) = format_file(path_str) {
                            eprintln!("❌ Error formatting file '{}': {}", path_str, e);
                            exit(1);
                        }
                        println!("✅ Successfully formatted file: {}", path_str);
                    } else if path.is_dir() {
                        let path_str = path.to_str().unwrap();
                        match format_dir(path_str, &self.exclude, self.recursive) {
                            Ok(count) => {
                                if count > 0 {
                                    println!("✅ Successfully formatted {} file{} in directory{}: {}", 
                                        count,
                                        if count == 1 { "" } else { "s" },
                                        if self.recursive { " (recursively)" } else { "" }, 
                                        path_str);
                                } else {
                                    println!("ℹ️ No files were formatted in directory{}: {}", 
                                        if self.recursive { " (recursively)" } else { "" }, 
                                        path_str);
                                }
                            },
                            Err(e) => {
                                eprintln!("❌ Error formatting directory '{}': {}", path_str, e);
                                exit(1);
                            }
                        }
                    } else {
                        eprintln!("❌ Invalid path: {}", path.display());
                        exit(1);
                    }
                } else {
                    eprintln!("❌ No command or path provided. Use `--help` for available commands.");
                    exit(1);
                }
            }
        }
    }
}