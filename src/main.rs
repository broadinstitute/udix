mod cli;

use udix::run;
use std::process::exit;
use udix::error::Error;

fn main() {
    match do_run() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {error}");
            exit(1)
        }
    }
}

fn do_run() -> Result<(), Error> {
    run(cli::get_selection()?)
}
