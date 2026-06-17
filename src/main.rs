use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::error::Error;
mod parser;


fn generate_html_document(body_content: &str) -> String {
    const CSS_STYLE: &str = include_str!("../style.css");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Converted Document</title>
    <style>
{}
    </style>
</head>
<body>
{}
</body>
</html>
"#,
        CSS_STYLE, 
        body_content
    )
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {

    ///Input path
    input_path: PathBuf,

    ///Output path
    output_path: PathBuf,

    /// Enables watch mode
    #[arg(short, long)]
    watch: bool,
}


fn md2html(args: &CliArgs) -> Result<(), Box<dyn Error>> {

    let md_content = fs::read_to_string(&args.input_path)?;

    let html_output = parser::parse(&md_content);

    let final_output = generate_html_document(&html_output);

    fs::write(&args.output_path, final_output)?;

    println!("Successfully converted {:?} to {:?}.", args.input_path, args.output_path);

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    md2html(&args)?;

    Ok(())
}
