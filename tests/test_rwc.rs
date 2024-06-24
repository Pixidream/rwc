use rwc::{get_file_bytes, get_file_char_count, get_file_lines_count};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

fn get_book_path() -> PathBuf {
    PathBuf::from("assets/books/cyrus_of_artamene.txt")
}

fn get_wc_value_for_arg(wc_arg: &str) -> u64 {
    let output = Command::new("wc")
        .args([wc_arg, get_book_path().to_str().unwrap()])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

#[test]
fn test_bytes() {
    let wc_bytes = get_wc_value_for_arg("-c");
    let rwc_bytes = get_file_bytes(&get_book_path());

    assert_eq!(wc_bytes, rwc_bytes)
}

#[test]
fn test_char_count() {
    let wc_char_count = get_wc_value_for_arg("-m");
    let rwc_char_count = get_file_char_count(&get_book_path());

    assert_eq!(wc_char_count, rwc_char_count)
}

#[test]
fn test_lines_count() {
    let wc_lines_count = get_wc_value_for_arg("-l");
    let rwc_lines_count = get_file_lines_count(&get_book_path());

    assert_eq!(wc_lines_count, rwc_lines_count)
}
