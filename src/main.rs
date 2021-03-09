use std::env;
use std::process;
use std::process::Command;
use std::process::Output;
use std::time::Duration;
use std::thread;

struct Config {
    pause_seconds: u64,
    command: String,
    args: Vec<String>,
}

impl Config {
    fn new(mut args: env::Args)-> Result<Config, &'static str> {
        args.next();
        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a shell command"),
        };

        let (pause_seconds, command) = if first_arg == String::from("-i") {
            let error_message = "didn't get a whole number of seconds after `-i` flag";
            let pause_seconds: u64 = match args.next() {
                Some(arg) => {
                    match arg.parse() {
                        Ok(int) => int,
                        _ => return Err(error_message),
                    }
                },
                None => return Err(error_message),
            };
            let command = match args.next() {
                Some(command) => command,
                None => return Err("didn't get a shell command"),
            };

            (pause_seconds, command)
        } else {
            (2, first_arg)
        };

        let args: Vec<String> = args.collect();

        Ok(Config {
            pause_seconds,
            command,
            args,
        })
    }
}


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Execute command and print stdout and stderr
    loop {
        let output = execute_shell_command(config.command.as_str(), &config.args);
        println!("{}", String::from_utf8_lossy(&output.stderr));
        println!("{}", String::from_utf8_lossy(&output.stdout));
        thread::sleep(Duration::from_secs(config.pause_seconds));
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
