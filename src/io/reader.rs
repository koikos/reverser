use std::{fs::File, io::BufReader, io::Read};


pub fn read_raw_file_to_buffer(input_file: &str) -> std::io::Result<Vec<u8>> {
    // Open the file for reading in raw mode
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read the entire file as raw bytes
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
