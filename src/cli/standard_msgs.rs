pub fn print_version_and_exit() {
    println!("{}", get_version());
    std::process::exit(0);
}

pub fn print_info_end_exit() {
    println!("     Binary Name: {}", built_info::PKG_NAME);
    println!("         Version: {}", built_info::PKG_VERSION);
    println!("      Git Commit: {}", get_git_info().unwrap_or_default());
    println!("Compilation Date: {}", get_build_timestamp());
    println!("         Authors: {}", built_info::PKG_AUTHORS);
    std::process::exit(0);
}

// ------------------------------------------------------------------------------------------------ priv below

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn get_version() -> String {
    format!("{} {} ({})", built_info::PKG_NAME, built_info::PKG_VERSION, get_build_date())
}

fn get_git_info() -> Option<String> {
    Some((built_info::GIT_COMMIT_HASH_SHORT, built_info::GIT_DIRTY))
        .and_then(|(hash, dirty)| Some((hash?, dirty?)))
        .map(|(hash, dirty)| 
            format!("{}-{}", hash, if dirty { "dirty" } else { "clean" })
        )
}

fn get_build_date() -> String {
    built::util::strptime(built_info::BUILT_TIME_UTC).format("%Y-%m-%d").to_string()
}

fn get_build_timestamp() -> String {
    built::util::strptime(built_info::BUILT_TIME_UTC).to_string()
}
