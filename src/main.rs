extern crate peach_web;

use std::process;

fn main() {
    // handle errors returned from `run`
    if let Err(_) = peach_web::run() {
        process::exit(1);
    }
}
