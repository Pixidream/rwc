use rwc::{get_file_bytes, get_file_char_count};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

fn get_book_path() -> PathBuf {
    PathBuf::from("assets/books/cyrus_of_artamene.txt")
}

#[test]
fn test_bytes() {
    // getting bytes from real wc
    let output = Command::new("wc")
        .args(["-c", get_book_path().to_str().unwrap()])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let wc_bytes = stdout
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // getting bytes from rwc
    let rwc_bytes = get_file_bytes(&get_book_path());

    assert_eq!(wc_bytes, rwc_bytes)
}

#[test]
fn test_char_count() {
    // getting char count from wc
    let output = Command::new("wc")
        .args(["-m", get_book_path().to_str().unwrap()])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let wc_char_count = stdout
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // getting char count from rwc
    let rwc_char_count = get_file_char_count(&get_book_path());

    assert_eq!(wc_char_count, rwc_char_count)
}
