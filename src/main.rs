use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // if you want debug logs, provide "debug" as the 3rd argument
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Closure called by unwrap or else in case of Err in Result
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.debug {
        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);
    }

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

