mod cli_handler;
mod io;
mod line_reverser;

fn main() -> std::io::Result<()> {
    let options = cli_handler::arg_parser::parse();

    let input_file = options.input_path;
    let output_file = line_reverser::output_file::get_output_filename(&input_file);

    let buffer = io::file_reader::read_raw_file_to_buffer(&input_file)?;
    let reversed_lines = line_reverser::line_reverser::reverse_file_lines(buffer);
    io::file_writer::write_raw_buffer_to_file(&output_file, reversed_lines)?;

    Ok(())
}
