use std::env;
use std::process::Command;
use std::process::Output;

pub struct Config {
    pub pause_seconds: u64,
    pub command: String,
    pub args: Vec<String>,
    pub clear_screen: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a shell command"),
        };

        let (first_arg, clear_screen) = if first_arg == *"-c" {
            let first_arg = match args.next() {
                Some(arg) => arg,
                None => return Err("didn't get a shell command"),
            };
            (first_arg, true)
        } else {
            (first_arg, false)
        };

        let (pause_seconds, command) = if first_arg == *"-i" {
            let error_message = "didn't get a whole number of seconds after `-i` flag";
            let pause_seconds: u64 = match args.next() {
                Some(arg) => match arg.parse() {
                    Ok(int) => int,
                    _ => return Err(error_message),
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
            clear_screen,
        })
    }
}

/// Executes a shell command and returns output
pub fn execute_shell_command(command: &str, args: &[String]) -> Output {
    let args = args.to_vec();

    Command::new(command)
        .args(&args)
        .output()
        .expect("could not execute command")
}
