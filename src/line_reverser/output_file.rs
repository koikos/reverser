use std::path::{Path, PathBuf};

pub fn get_output_filename(input_path: &Path) -> PathBuf {
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