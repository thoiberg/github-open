// get git repo name with `git remote -v`
// extract repo name from the return
// get repo top level directory with `git rev-parse --show-toplevel`
// get the full path for the file (passed in as an arg)
// remove the top level directory from the path to get just the file path inside the repo
// combine that with the repo name to get a link to the file
// `open` it

use std::env;
use std::path::Path;

use github_open::*;

fn main() {
    // TODO: add graceful handling to deal with the wrong amount/type of args being passed in
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);
    // TODO: check if file exists, if not print and exit with non-zero code
    // TODO: implement proper handling of Err result
    let full_file_path = file_path.canonicalize().unwrap();

    let git_remotes = get_git_remotes().unwrap();

    let git_repos = get_git_repo_from_remotes(git_remotes);

    let git_root = get_git_root();

    // Example output:
    // https://github.com/thoiberg/github-open/blob/master/src/main.rs
    let full_file_string = full_file_path.to_str().unwrap();
    let repo_relative_path = full_file_string.replace(&git_root[..].trim(), "");

    let mut github_link = String::from("https://github.com/");
    github_link.push_str(&git_repos[0][..]);
    github_link.push_str("/blob/master");
    github_link.push_str(&repo_relative_path[..]);

    println!("github_link: {}", github_link);
}
