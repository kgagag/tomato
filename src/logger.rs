// my_logger.rs
extern crate log;
extern crate env_logger;

use log::{error, info, warn};
use std::env;

pub struct MyLogger;

impl MyLogger {
    pub fn init() {
        env_logger::init();
    }

    pub fn info(message: &str) {
        info!("{}", message);
    }

    pub fn warn(message: &str) {
        warn!("{}", message);
    }

    pub fn error(message: &str) {
        error!("{}", message);
    }

    pub fn set_log_level(level: &str) {
        env::set_var("RUST_LOG", level);
    }
}
