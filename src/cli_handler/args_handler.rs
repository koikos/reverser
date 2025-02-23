use super::parsed_args::ParsedArgs;
use clap::{arg, command, value_parser, Command};
use std::path::PathBuf;

pub fn parse_args() -> ParsedArgs {
    let matches = build_parser().get_matches();

    return ParsedArgs {
        input_path: matches
            .get_one::<PathBuf>("INPUT_FILE")
            .unwrap()                                       // safe to unwrap because it is required
            .to_path_buf(),
    };
}


fn build_parser() -> Command{
    return command!()
        .arg(
            arg!(<INPUT_FILE> "The file to reverse")            // '<>' indicate required argument
                .index(1)
                .value_parser(value_parser!(PathBuf)),              // parse the value to PathBuf (with validation)
        )
}


