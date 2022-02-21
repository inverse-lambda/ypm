extern crate log; // error, warn, info, debug, trace
use log::{Record, Level, Metadata};

pub static LOGGER: Logger = Logger;
pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {        
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Info => println!("{}", record.args()),
                _ => println!("{}: {}", record.level(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}
