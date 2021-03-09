use std::env;
use std::process;
use std::time::Duration;
use std::thread;

use watch::execute_shell_command;
use watch::Config;

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
