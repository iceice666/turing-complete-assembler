use clap::Parser;



pub mod parser;
pub mod datakind;

#[derive(Parser)]
struct CliCmd {
    /// The path to the file to be assembled
    path: String
}

fn main() {
    let cli = CliCmd::parse();
    let input_filename = cli.path;
    let output_filename = input_filename.replace(".asm", ".bin");

    let content = std::fs::read_to_string(input_filename).expect("Failed to read the input file");

    let result = parser::parse(content);

    std::fs::write(output_filename, result.join("\n")).expect("Failed to write to the output file");
}