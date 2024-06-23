use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn get_file_bytes(file: &PathBuf) -> u64 {
    let file = File::open(file).unwrap_or_else(|_| {
        panic!(
            "[get_file_bytes] Can't open the provided file: '{:#?}'",
            file
        )
    });
    file.metadata().unwrap().len()
}

pub fn get_file_char_count(file: &PathBuf) -> u64 {
    let mut count = 0u64;

    let mut buf_reader = BufReader::new(File::open(file).unwrap_or_else(|_| {
        panic!(
            "[get_file_char_count] Can't open the provided: '{:#?}'",
            file
        )
    }));
    let mut buf = Vec::<u8>::new();

    while buf_reader
        .read_until(b'\n', &mut buf)
        .expect("[get_file_char_count] Can't read_until buffer.")
        != 0
    {
        let line = String::from_utf8(buf).expect("[get_file_char_count] Can't convert Vec<u8> buffer to String. Is your file UTF-8 encoded ?");
        for _ in line.chars() {
            count += 1;
        }

        buf = line.into_bytes();
        buf.clear();
    }

    count
}
