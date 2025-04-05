//----------------------------------------------//
//                                              //
// languages.rs - language management functions //
//                                              //
//----------------------------------------------//


use crate::config::{load_config, save_config, Language};

pub fn add_language(
    name: &str,
    single_line_comment: &str,
    extensions: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config();

    if config.languages.iter().any(|lang| lang.name == name) {
        println!("Language {} already exists.", name);
        return Ok(());
    }

    config.languages.push(Language {
        name: name.to_string(),
        single_line_comment: single_line_comment.to_string(),
        extensions,
    });

    save_config(&config);
    println!("Added language: {}", name);
    Ok(())
}

pub fn list_languages() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();

    for lang in config.languages {
        println!("{} → {}", lang.name, lang.single_line_comment);
    }

    Ok(())
}