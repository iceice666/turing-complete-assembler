use clap::Parser;

pub mod datakind;
pub mod parser;

#[derive(Parser)]
struct CliOptions {
    /// The path to the file to be assembled
    path: String,

    /// Preventing the assembler from copying comments to the output
    #[arg(long)]
    no_comments: bool,
}

fn main() {
    let cli = CliOptions::parse();
    let input_filename = &cli.path;
    let output_filename = input_filename.replace(".asm", ".bin");

    let content = std::fs::read_to_string(input_filename).expect("Failed to read the input file");

    let result = parser::parse(content, cli);

    match result {
        Ok(result) => {
            println!("Assembled successfully");
            std::fs::write(output_filename, result.join("\n"))
                .expect("Failed to write to the output file");
        }
        Err(err) => {
            eprintln!("Failed to assemble: {}", err);
        }
    }
}
