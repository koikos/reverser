













// use clap::{Arg, Command};
// use std::path::PathBuf;

// pub fn handle_args(matches: clap::ArgMatches) {
//     options = parse_args();
    
//     if V --> print_version_end_exit();
//     if I --> print_info_end_exit();
//     if H --> print_help_end_exit();

//     return input_file_name;
// }
    






//     if let Some(version) = matches.get_one::<bool>("version").copied() {
//         if version {
            
//             return;
//         }
//     }

//     if let Some(info) = matches.get_one::<bool>("info").copied() {
//         if info {
//             println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
//             println!("Binary Name: {}", env!("CARGO_PKG_NAME"));
//             println!("Binary Location: {}", env::current_exe().unwrap().display());
//             println!("Version: {}", env!("CARGO_PKG_VERSION"));
//             println!("Compilation Date: {}", env!("CARGO_PKG_VERSION"));
//             return;
//         }
//     }

//     if let Some(file_path) = matches.get_one::<PathBuf>("file") {
//         info!("Provided file path: {}", file_path.display());
//     }
// }

// fn parse_args() -> clap::ArgMatches {
//     Command::new(env!("CARGO_PKG_NAME"))
//         .about(env!("CARGO_PKG_DESCRIPTION"))
//         .arg(
//             Arg::new("file")
//                 .help("Path to the file")
//                 .required(true)
//                 .value_parser(clap::value_parser!(PathBuf)),
//         )
//         .arg(
//             Arg::new("info")
//                 .long("info")
//                 .help("Display program metadata")
//                 .action(clap::ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("version")
//                 .short('v')
//                 .long("version")
//                 .help("Display version information")
//                 .action(clap::ArgAction::SetTrue),
//         )
//         .get_matches()
// }
