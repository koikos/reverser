use std::process::Command;
use chrono::Utc;

fn main() {
    expose_build_profile();
    expose_build_timestamp();
    expose_git_info();
}

fn expose_build_profile() {
    let profile = std::env::var("PROFILE").unwrap_or("DEBUG".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    println!("cargo:rerun-if-env-changed=PROFILE");
}

fn expose_build_timestamp() {
    let format = "%Y-%m-%d %H:%M:%S%.3f UTC";   // Format: 2021-08-31 23:59:59.999 UTC
    let timestamp = Utc::now().format(format).to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    }

fn expose_git_info() {
    let git_hash = get_git_hash().unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    let git_status = is_git_workspace_clean().unwrap_or(false);
    println!("cargo:rustc-env=GIT_WORKSPACE_CLEAN={}", git_status);

    // Regenerate if any git-related changes occur
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}

fn get_git_hash() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err("Failed to get git hash".into())
    }
}

fn is_git_workspace_clean() -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()?;

    if output.status.success() {
        Ok(output.stdout.is_empty())
    } else {
        Err("Failed to get git status".into())
    }
}