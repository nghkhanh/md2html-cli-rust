use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::error::Error;
mod parser;


fn generate_html_document(body_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Converted Document</title>
</head>
<body>
{}
</body>
</html>
"#, 
        body_content
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let md_content = fs::read_to_string(&args.input_path)?;

    let html_output = parser::parse(&md_content);

    let final_output = generate_html_document(&html_output);

    fs::write(&args.output_path, final_output)?;

    println!("Successfully converted {:?} to {:?}.", args.input_path, args.output_path);

    Ok(())
}


// fn parse_md(content: &str) -> String {
//     content.to_string()
// }


#[derive(Parser, Debug)]
struct CliArgs {
    input_path: PathBuf,
    output_path: PathBuf,
}
