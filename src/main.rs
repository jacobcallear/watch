use std::env;
use std::process::Command;
use std::process::Output;
use std::time::Duration;
use std::thread;

fn main() {
    // Parse command line args
    let mut args = env::args().skip(1);
    let command = args.next().unwrap();
    let args: Vec<String> = args.collect();

    // Execute command and print stdout
    loop {
        let output = execute_shell_command(command.as_str(), &args);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        println!("{}", String::from_utf8_lossy(&output.stdout));
        thread::sleep(Duration::from_secs(2));
    }
}

/// Executes a shell command and returns output
fn execute_shell_command(command: &str, args: &Vec<String>) -> Output {
    let args = args.to_vec();

    Command::new(command)
        .args(&args)
        .output()
        .expect("could not execute command")
}
