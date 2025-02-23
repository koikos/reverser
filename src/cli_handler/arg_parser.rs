use super::options::Options;
use clap::{arg, command, Command};
use std::path::PathBuf;

pub fn parse() -> Options {
    let matches = build_parser().get_matches();

    if matches.get_flag("info") {
        print_info();
        std::process::exit(0);
    }

    return Options {
        input_path: matches
            .get_one::<PathBuf>("INPUT_FILE_PATH")
            .unwrap_or_else(|| input_file_not_provided_error())
            .to_path_buf(),
    };
}

fn build_parser() -> Command {
    return command!()
        .arg(
            arg!([INPUT_FILE_PATH] "The file to reverse")
                .index(1)
                .value_parser(path_parser_with_validation),
        )
        .arg(
            arg!(-i --info "Print build information message")
                .action(clap::ArgAction::SetTrue),
        );
}

fn path_parser_with_validation(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    if !path.exists() {
        return Err(format!("Path '{}' does not exist", value));
    } else if path.is_dir() {
        return Err(format!("Path '{}' points to a directory", value));
    } else {
        return Ok(path);
    }
}

fn input_file_not_provided_error() -> ! {
    command!()
        .error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "Error: No input file provided",
        )
        .exit();
}

fn print_info() {
    println!("     Binary Name: {}", env!("CARGO_PKG_NAME"));
    println!("         Version: {}", env!("CARGO_PKG_VERSION"));
    println!("   Build Profile: {}", env!("BUILD_PROFILE"));
    println!(" Build Timestamp: {}", env!("BUILD_TIMESTAMP"));
    println!("      Git Commit: {}", get_git_commit_msg());
    println!("         Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("         License: {}", env!("CARGO_PKG_LICENSE"));
}

fn get_git_commit_msg() -> String {
    let git_clean = env!("GIT_WORKSPACE_CLEAN") == "true";
    if git_clean {
        return format!("{}", env!("GIT_HASH"));
    } else {
        return format!("{} (dirty)", env!("GIT_HASH"));
    }
}
