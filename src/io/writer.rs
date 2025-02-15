use std::{fs::File, io::BufWriter, io::Write};

pub fn write_raw_lines_to_file(output_file: &str, lines: &[&[u8]]) -> std::io::Result<()> {
    // Open the output file for writing
    let file = File::create(output_file)?;
    let mut writer: BufWriter<File> = BufWriter::new(file);

    // Write the reversed lines back
    for line in lines {
        writer.write_all(line)?;
    }

    Ok(())
}