extern crate log; // error, warn, info, debug, trace
use log::{Record, Level, Metadata};

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // filter solely via set_max_level set value!
        //metadata.level() <= Level::Info
        true
    }

    fn log(&self, record: &Record) {        
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Info => println!("{}", record.args()),
                //Level::Trace => println!("VV: ", record.args()), ...
                _ => println!("({}) {}", record.level(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * Init logger and set the maximum log level
 */
pub fn init(log_level: log::LevelFilter) {
    if let Err(e) = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level)) {
        panic!("Unable to set log level to {}: {:?}", log_level, e)
    }
    log::trace!("Set log level to: {}", log_level);
}