use pulldown_cmark::{html, Options, Parser};
use std::fs;

pub fn create_slides(file: &str) -> std::io::Result<Vec<String>> {
    let html = parse_markdown(file);
    let slides = html.split("<hr />").map(|s| s.to_string()).collect();

    Ok(slides)
}

pub fn safe_get_current_slide(slides: &Vec<String>, idx: usize) -> String {
    if idx >= slides.len() {
        slides[slides.len() - 1].clone()
    } else {
        slides[idx].clone()
    }
}

pub fn add_controls(html: &str, slides: &Vec<String>) -> std::io::Result<String> {
    let controls = &format!("const slideLen = {};", slides.len());
    let html_output = html.replace("<!-- controls -->", controls);

    Ok(html_output)
}

pub fn add_slide(slide: &str, slides: &Vec<String>) -> std::io::Result<String> {
    let tmpl = fs::read_to_string("tmpl.html").unwrap();
    let html_output = tmpl.replace("<!-- content -->", &slide);
    let html_output = add_controls(&html_output, slides)?;

    Ok(html_output)
}

fn parse_markdown(file: &str) -> String {
    let markdown_input = std::fs::read_to_string(file).expect("Unable to read file");

    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
