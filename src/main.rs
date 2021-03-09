use std::env;
use std::process;
use std::thread;
use std::time::Duration;

use watch::execute_shell_command;
use watch::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Execute command and print stdout and stderr
    loop {
        if config.clear_screen {
            print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }

        let output = execute_shell_command(config.command.as_str(), &config.args);
        print!("{}", String::from_utf8_lossy(&output.stderr));
        print!("{}", String::from_utf8_lossy(&output.stdout));
        thread::sleep(Duration::from_secs(config.pause_seconds));
    }
}
