mod cli;
mod io;
mod line_reverser;

fn main() -> std::io::Result<()> {
    let input_file = r"./examples/example.csv";
    let output_file = r"./examples/example-reversed.csv";

    let buffer = io::file_reader::read_raw_file_to_buffer(input_file)?;
    let reversed_lines = line_reverser::line_reverser::reverse_file_lines(buffer);
    io::file_writer::write_raw_buffer_to_file(output_file, reversed_lines)?;
    
    cli::standard_msgs::print_info_end_exit();
    cli::standard_msgs::print_version_and_exit();

    Ok(())
}
