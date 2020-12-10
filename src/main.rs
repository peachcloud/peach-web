//! Initialize the logger and run the application, catching any errors.

use std::process;

use log::error;

fn main() {
    // initialize the logger
    env_logger::init();

    // handle errors returned from `run`
    if let Err(e) = peach_web::run() {
        error!("Application error: {}", e);
        process::exit(1);
    }
}
