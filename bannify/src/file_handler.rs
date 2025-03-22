//#// file_handler.rs - Files System Functions

use crate::{banner::create_banner, config::load_config};
use glob::glob;
use std::fs;

pub fn format_file(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let content = fs::read_to_string(file)?;
    let extension = file.split('.').last().unwrap_or("");

    for lang in &config.languages {
        if lang.extensions.contains(&extension.to_string()) {
            let formatted = content
                .lines()
                .map(|line| {
                    if let Some(content) = line.strip_prefix(&format!(
                        "{}#{}",
                        lang.single_line_comment, lang.single_line_comment
                    )) {
                        create_banner(content.trim(), &lang.single_line_comment)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            fs::write(file, formatted)?;
            println!("Formatted {}", file);
            return Ok(());
        }
    }

    println!("No matching language for {}", file);
    Ok(())
}

pub fn format_dir(dir: &str, exclude: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for entry in glob(&format!("{}/**/*", dir))? {
        let path = entry?;
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if !exclude.contains(&ext.to_string()) {
            format_file(path.to_str().unwrap())?;
        }
    }

    Ok(())
}
