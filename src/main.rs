use std::process::Command;
use std::process::Output;

fn main() {
    let output = execute_shell_command("git", &vec!["status"]);

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn execute_shell_command(command: &str, args: &Vec<&str>) -> Output {
    let args = args.to_vec();

    Command::new(command)
        .args(&args)
        .output()
        .expect("could not execute command")
}
