mod cli;
mod io;
mod line_reverser;

fn main() -> std::io::Result<()> {
    let input_file = r"./examples/2025-02-13.csv";
    let output_file = r"./examples/2025-02-13-reversed.csv";
    line_reverser::line_reverser::reverse_file_lines(input_file, output_file)?;

    cli::standard_msgs::print_info_end_exit();
    cli::standard_msgs::print_version_and_exit();

    Ok(())
}
