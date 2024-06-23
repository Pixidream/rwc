use clap::Parser;
use rwc::{get_file_bytes, get_file_char_count};
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
}

fn main() {
    let args = Args::parse();

    if args.bytes {
        println!(
            "{} {}",
            get_file_bytes(&args.file),
            args.file.file_name().unwrap().to_str().unwrap()
        );
    }

    if args.chars {
        println!(
            "{} {}",
            get_file_char_count(&args.file),
            args.file.file_name().unwrap().to_str().unwrap()
        )
    }
}
