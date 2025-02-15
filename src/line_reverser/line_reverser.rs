use crate::io::{reader, writer};

pub fn reverse_file_lines(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let buffer = reader::read_raw_file_to_buffer(input_file)?;

    // Split the buffer into lines while keeping line endings intact
    let mut lines: Vec<&[u8]> = buffer.split_inclusive(|&b| b == b'\n').collect();
    lines.reverse();

    writer::write_raw_lines_to_file(output_file, &lines);
    Ok(())
}
