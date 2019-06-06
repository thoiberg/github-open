extern crate regex;

use regex::Regex;
use std::process::Command;

// Check if the status is successful then either return Ok with the String or Err
pub fn get_git_remotes() -> std::io::Result<String> {
    let output = Command::new("git").arg("remote").arg("-v").output();
    let unwrapped_output = output?;

    if !unwrapped_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "unable to get github remotes",
        ));
    }

    Ok(String::from_utf8_lossy(&unwrapped_output.stdout).to_string())
}

// TODO: Return a Result instead
// Check if the status is successful then either return Ok with the String or Err
pub fn get_git_repo_from_remotes(git_remotes: String) -> Vec<String> {
    // origin	git@github.com:thoiberg/github-open.git (fetch)
    // origin	git@github.com:thoiberg/github-open.git (push)

    let re = Regex::new(r".+:(.+)\.git").unwrap();
    let mut captures: Vec<String> = Vec::new();
    for cap in re.captures_iter(&git_remotes[..]) {
        captures.push(String::from(&cap[1]));
    }

    return captures;
}

pub fn get_git_root() -> String {
    // git rev-parse --show-toplevel
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output();
    let unwrapped_output = output.unwrap();

    return String::from(String::from_utf8_lossy(&unwrapped_output.stdout));
}
