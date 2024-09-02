/**
 * Let's assume that command or params which users input looks like this:
 * `cargo run -- searchstring example-filename.txt`
 */
use std::env;
use std::process;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) =  mini_grep::run(config) {
        eprintln!("Application exit with error {e}");
        process::exit(1);
    }
}


 