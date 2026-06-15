use clap::Parser;
use std::path::PathBuf;

fn main() {
    let args = CliArgs::parse();

    println!("Parsed arguments: {:?}", args);
    println!("Input file path: {:?}", args.input_path);
    println!("Output file path: {:?}", args.output_path);
}


#[derive(Parser, Debug)]
struct CliArgs {
    input_path: PathBuf,
    output_path: PathBuf,
}
