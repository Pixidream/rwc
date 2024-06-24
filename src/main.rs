use clap::Parser;
use rwc::{get_file_bytes, get_file_char_count, get_file_lines_count};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "1.0", name = "rwc", about = "rwc - Rust implementation of wc. print newline, word, and byte counts for each file", long_about = None)]
struct Args {
    file: PathBuf,

    /// Print the bytes counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Print the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    /// Print the newline counts
    #[arg(short = 'l', long)]
    lines: bool,
}

fn main() {
    let args = Args::parse();

    let file_name = args.file.file_name().unwrap().to_str().unwrap();

    match (args.bytes, args.chars, args.lines) {
        (false, false, false) => println!(
            "  {} word {} {}",
            get_file_lines_count(&args.file),
            get_file_bytes(&args.file),
            file_name
        ),
        (true, false, false) => println!("{} {}", get_file_bytes(&args.file), file_name),
        (false, true, false) => println!("{} {}", get_file_char_count(&args.file), file_name),
        (false, false, true) => println!("{} {}", get_file_lines_count(&args.file), file_name),
        _ => println!("Invalid combination of arguments."),
    }
}
