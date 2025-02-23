use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use encoding_rs::WINDOWS_1250;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if !validate_args(&args) {
        return Ok(());
    }

    let input_path = &args[1];
    let path = Path::new(input_path);
    
    if !validate_path(&path) {
        return Ok(());
    }

    // read and reverse lines //
    
    let output_path = get_output_filename(path);
    if !confirm_overwrite(&output_path)? {
        return Ok(());
    }

    // save file
    println!("Saved file: {}", output_path.display());
    Ok(())
}

fn validate_args(args: &[String]) -> bool {
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return false;
    }
    if args.len() != 2 || args[1] == "--help" || args[1] == "-h" {
        eprintln!("Usage: {} <file_path>", args[0]);
        return false;
    }
    true
}

fn validate_path(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        eprintln!("Error: File not found or not a valid file.");
        return false;
    }
    true
}


fn confirm_overwrite(output_path: &Path) -> io::Result<bool> {
    if output_path.exists() {
        println!("Output file {} already exists. Overwrite? (y/N): ", output_path.display());
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Operation aborted.");
            return Ok(false);
        }
    }
    Ok(true)
}


fn get_output_filename(input_path: &Path) -> PathBuf {
    let mut output_path = input_path.to_path_buf();
    if let Some(extension) = input_path.extension() {
        output_path.set_extension(extension);
    }
    output_path.set_file_name(format!(
        "{}_reversed.{}",
        input_path.file_stem().unwrap().to_string_lossy(),
        input_path.extension().unwrap().to_string_lossy()
    ));
    output_path
}