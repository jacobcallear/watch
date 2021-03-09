use std::env;
use std::process::Command;
use std::process::Output;

fn main() {
    // Parse command line args
    let mut args = env::args().skip(1);
    let command = args.next().unwrap();
    let args: Vec<String> = args.collect();

    // Execute command and print stdout
    let output = execute_shell_command(command.as_str(), &args);
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

/// Executes a shell command and returns output
fn execute_shell_command(command: &str, args: &Vec<String>) -> Output {
    let args = args.to_vec();

    Command::new(command)
        .args(&args)
        .output()
        .expect("could not execute command")
}
