use std::env;
use std::process::Command;
use std::process::Output;
use std::time::Duration;
use std::thread;

fn main() {
    let mut args = env::args().skip(1);

    // Get pause time if -i flag given, otherwise default to 2 seconds
    // Get passed shell command
    let first_arg = args.next().expect("expected a shell command");
    let (pause_seconds, command) = if first_arg == String::from("-i") {
        let error_message = "expected a whole number of seconds after `-i` flag";
        let pause_seconds: u64 = args.next()
            .expect(error_message)
            .parse().expect(error_message);
        (pause_seconds, args.next().unwrap())
    } else {
        (2, first_arg)
    };

    let args: Vec<String> = args.collect();

    // Execute command and print stdout and stderr
    loop {
        let output = execute_shell_command(command.as_str(), &args);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        println!("{}", String::from_utf8_lossy(&output.stdout));
        thread::sleep(Duration::from_secs(pause_seconds));
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
