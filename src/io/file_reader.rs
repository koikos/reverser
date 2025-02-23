use super::types::CharactersSerie;
use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

pub fn read_raw_file_to_buffer(input_file: &PathBuf) -> std::io::Result<CharactersSerie> {
    let file = open_file_for_reading_in_raw_mode(input_file)?;
    let buffer = read_file_as_raw_bytes(file)?;
    Ok(buffer)
}

fn open_file_for_reading_in_raw_mode(input_file: &PathBuf) -> std::io::Result<File> {
    File::open(input_file)
}

fn read_file_as_raw_bytes(file: File) -> std::io::Result<CharactersSerie> {
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}
