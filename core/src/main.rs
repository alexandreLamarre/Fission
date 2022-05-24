#![warn(dead_code)]
mod metrics;

use core::setup_logger;

fn main() {
    println!("Running Fission ....");
    println!("Setting up logger ...");
    match setup_logger("main") {
        Ok(_) => {
            println!("Logger setup successfully");
        }
        Err(e) => {
            panic!("Error setting up logger: {}", e);
        }
    }
    println!("... Fission finished!");
}
