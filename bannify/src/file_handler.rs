//------------------------------------------//
//                                          //
// file_handler.rs - Files System Functions //
//                                          //
//------------------------------------------//


use crate::{banner::create_banner, config::load_config};
use glob::Pattern;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path};
use walkdir::WalkDir;

/// Format a single file, returns true if the file was actually modified
pub fn format_file(file: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let config = load_config();
    
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) if e.kind() == ErrorKind::InvalidData => {
            // Skip files with invalid UTF-8 data
            return Ok(false);
        },
        Err(e) => return Err(Box::new(e)),
    };
    
    let extension = file.split('.').last().unwrap_or("");

    for lang in &config.languages {
        if lang.extensions.contains(&extension.to_string()) {
            let mut was_modified = false;
            let formatted = content
                .lines()
                .map(|line| {
                    if let Some(content) = line.strip_prefix(&format!(
                        "{}#{}",
                        lang.single_line_comment, lang.single_line_comment
                    )) {
                        was_modified = true;
                        create_banner(content.trim(), &lang.single_line_comment)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            if was_modified {
                fs::write(file, formatted)?;
                return Ok(true);
            }
            
            return Ok(false);
        }
    }

    Ok(false)
}

pub fn format_dir(dir: &str, exclude: &[String], recursive: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let mut formatted_count = 0;
    
    if recursive {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) {
                
            if should_process_file(&entry.path(), exclude) && is_supported_file(&entry.path()) {
                match format_file(entry.path().to_str().unwrap()) {
                    Ok(true) => {
                        println!("✅ Formatted file: {}", entry.path().display());
                        formatted_count += 1;
                    },
                    Ok(false) => {
                    },
                    Err(e) => {
                        eprintln!("⚠️ Skipping {}: {}", entry.path().display(), e);
                    }
                }
            }
        }
    } else {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && should_process_file(&path, exclude) && is_supported_file(&path) {
                match format_file(path.to_str().unwrap()) {
                    Ok(true) => {
                        println!("✅ Formatted file: {}", path.display());
                        formatted_count += 1;
                    },
                    Ok(false) => {
                    },
                    Err(e) => {
                        eprintln!("⚠️ Skipping {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    Ok(formatted_count)
}

fn should_process_file(path: &Path, exclude: &[String]) -> bool {
    // Skip hidden files
    if path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false) 
    {
        return false;
    }
    
    // Skip files with excluded extensions
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        if exclude.contains(&ext.to_string()) {
            return false;
        }
    }
    
    // Skip files matching excluded patterns
    if let Some(path_str) = path.to_str() {
        for pattern in exclude {
            if pattern.ends_with('/') || pattern.ends_with('\\') {
                let dir_pattern = pattern.trim_end_matches(|c| c == '/' || c == '\\');
                if path_str.contains(dir_pattern) {
                    return false;
                }
            } else if Pattern::new(pattern).map(|p| p.matches(path_str)).unwrap_or(false) {
                return false;
            }
        }
    } else {
        return false;
    }
    
    true
}

pub fn is_supported_file(file: &Path) -> bool {
    let config = load_config();
    
    let extension = file.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
        
    config.languages.iter().any(|lang| lang.extensions.contains(&extension.to_string()))
}