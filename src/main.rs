// get git repo name with `git remote -v`
// extract repo name from the return
// get repo top level directory with `git rev-parse --show-toplevel`
// get the full path for the file (passed in as an arg)
// remove the top level directory from the path to get just the file path inside the repo
// combine that with the repo name to get a link to the file
// `open` it

use std::process::Command;

fn get_git_remotes() -> String {
    let output = Command::new("git").arg("remote").arg("-v").output();
    let unwrapped_output = output.unwrap();
    println!("output: {:?}", unwrapped_output);
    println!("status: {}", unwrapped_output.status);
    println!("was successful?: {}", unwrapped_output.status.success());
    println!("stdout: {:?}", unwrapped_output.stdout);

    return String::from(String::from_utf8_lossy(&unwrapped_output.stdout));
}

fn main() {
    let git_remotes = get_git_remotes();

    println!("git remotes: {}", git_remotes);
}
