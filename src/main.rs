// get git repo name with `git remote -v`
// extract repo name from the return
// get repo top level directory with `git rev-parse --show-toplevel`
// get the full path for the file (passed in as an arg)
// remove the top level directory from the path to get just the file path inside the repo
// combine that with the repo name to get a link to the file
// `open` it

extern crate regex;

use std::process::Command;
use regex::Regex;

// TODO: Return a Result instead
// Check if the status is successful then either return Ok with the String or Err
fn get_git_remotes() -> String {
    let output = Command::new("git").arg("remote").arg("-v").output();
    let unwrapped_output = output.unwrap();
    println!("output: {:?}", unwrapped_output);
    println!("status: {}", unwrapped_output.status);
    println!("was successful?: {}", unwrapped_output.status.success());

    return String::from(String::from_utf8_lossy(&unwrapped_output.stdout));
}

fn get_git_repo_from_remotes(git_remotes: String) -> Vec<String> {
    // origin	git@github.com:thoiberg/github-open.git (fetch)
    // origin	git@github.com:thoiberg/github-open.git (push)

    let re = Regex::new(r".+:(.+)\.git").unwrap();
    let mut captures: Vec<String> = Vec::new();
    for cap in re.captures_iter(&git_remotes[..]) {
        captures.push(String::from(&cap[1]));
    };

    return captures;
}

fn get_git_root() -> String {
    // git rev-parse --show-toplevel
    let output = Command::new("git").arg("rev-parse").arg("--show-toplevel").output();
    let unwrapped_output = output.unwrap();

    return String::from(String::from_utf8_lossy(&unwrapped_output.stdout));
}

fn main() {
    let git_remotes = get_git_remotes();
    println!("git remotes: {}", git_remotes);

    let git_repos = get_git_repo_from_remotes(git_remotes);
    println!("The captured repo names are: {:?}", git_repos);

    let git_root = get_git_root();
    println!("The git root is: {}", git_root);
}
