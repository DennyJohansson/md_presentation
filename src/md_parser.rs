use pulldown_cmark::{html, Options, Parser};
use std::fs;

pub fn read_md(file: &str) -> std::io::Result<String> {
    let tmpl = fs::read_to_string("tmpl.html").unwrap();
    let markdown = parse_markdown(&format!("content/{}", file));
    let tmpl = tmpl.replace("<!-- content -->", &markdown);

    Ok(tmpl)
}

fn parse_markdown(file: &str) -> String {
    let markdown_input = std::fs::read_to_string(file).expect("Unable to read file");

    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // html_output = html_output.replace("<h1>", "<h1 class='text-4xl font-extrabold text-white pt-12 md:text-5xl lg:text-6xl sm:px-16 xl:px-48'>");
    // html_output = html_output.replace("<h2>", "<h2 class='text-3xl font-extrabold text-white pt-12 md:text-4xl lg:text-5xl sm:px-16 xl:px-48'>");
    // html_output = html_output.replace("<p>", "<p class='text-lg font-normal mb-4 text-gray-400 lg:text-xl sm:px-16 xl:px-48'>");
    // println!("{}", html_output);

    html_output
}
