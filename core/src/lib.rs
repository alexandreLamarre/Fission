#![warn(dead_code)]

extern crate fern;
extern crate threadpool;

use log::{debug, error, info, trace, warn};

mod api;
mod dataset;
mod metrics;
mod node;
mod reader;

/// @param file_name : name of the file, will be suffixed with .log
pub fn setup_logger(file_name: &str) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(format!("{}.log", file_name))?)
        .apply()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_setup_logger() {
        assert!(setup_logger("lib-test.log").is_ok());
    }
}
