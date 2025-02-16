use std::{fs::File, io::BufWriter, io::Write};
use super::types::CharactersSerie;

pub fn write_raw_buffer_to_file(output_file: &str, buffer: CharactersSerie) -> std::io::Result<()> {
    let file = open_file_for_writing_in_raw_mode(output_file)?;
    write_raw_bytes_to_file(file, buffer)?;
    Ok(())
}

fn open_file_for_writing_in_raw_mode(output_file: &str) -> std::io::Result<File> {
    File::create(output_file)
}

fn write_raw_bytes_to_file(file: File, buffer: CharactersSerie) -> std::io::Result<()> {
    let mut writer: BufWriter<File> = BufWriter::new(file);
    writer.write(&buffer)?;
    writer.flush()?;
    Ok(())
}