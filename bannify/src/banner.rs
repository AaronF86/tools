//------------------------------------------//
//                                          //
// banner.rs - Banner Formatter             //
//                                          //
//------------------------------------------//


pub fn create_banner(content: &str, comment: &str) -> String {
    let width = content.len().max(40);
    let separator = format!("{comment}{:-<width$}{comment}\n", "", width = width + 2);
    let _padding = (width - content.len()) / 2;

    format!(
        "{separator}\
         {comment} {:<width$} {comment}\n\
         {comment} {content:<width$} {comment}\n\
         {comment} {:<width$} {comment}\n\
         {separator}",
        "",
        "",
        content = content,
        width = width
    )
}