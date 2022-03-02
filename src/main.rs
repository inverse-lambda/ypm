use std::env;
use thiserror::Error;

mod parser;
mod util;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

////////////////////////////////////////////////////////////////////////////////////////////////////

fn print_help() {
  println!("
╔════════════════════════╗
║ Inverse Lambda Manager ║
╟────────────────────────╢
║ Version:  {: >12} ║
╚════════════════════════╝

USAGE:  
        y [OPTIONS] [TOOL]

OPTIONS:
        -v, --verbose               Use verbose output (-vv very verbose output)
        -h, --help                  Prints this information (also without param)

TOOL:
        build, b                    Compile the current package
        build, b  [PATH]            Compile the denoted package
        
", VERSION.unwrap_or("unknown"));
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Error)]
pub enum MainError {

    #[error("No such tool: `{tool:}`")]
    UnknownToolError {tool: String},

    #[error(transparent)]
    ParseError(#[from] parser::ParseError)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tool_args_pos = 0;
    let mut log_level: log::LevelFilter = log::LevelFilter::Info;
    //set_max_log_level(log::LevelFilter::Trace);
    //let tool = args.iter().position(|a| !a.starts_with('-'));
    //log::info!("Tool Pos: {:?}, Args: {:?}",tool, args);
    
    // if log::log_enabled!(log::Level::Info) {
    //     println!("- max level: {:?} -", log::max_level());
    // }

    // [OPTIONS] (several)
    for i in 1..args.len() { 
        match args[i].as_str() {
            "-v" | "--verbose"  => { log_level = log::LevelFilter::Debug; }, // -v  also shows debug messages
            "-vv"               => { log_level = log::LevelFilter::Trace; }, // -vv also shows trace messages
            _ if !args[i].starts_with('-') => { tool_args_pos = i; break; },
            _ => { }
        }
    }

    // [LOGGING] Set log level
    util::log::init(log_level);

    // [TOOL] (only one)
    if let Err(e) = select_tool(args, tool_args_pos) {
        
        // Simple error message
        log::error!("{:}", e.to_string());

        // Full error (only in non-optimized builds)
        if cfg!(debug_assertions) { //&& log::log_enabled!(log::Level::Debug) {
            log::error!("{:#?}", e); 
        } 
        
        // Exit non-zero code
        std::process::exit(1)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/**
 * Select which tool to use
 */
fn select_tool(args: Vec<String>, tool_args_pos: usize) -> Result<(), MainError> { 
    
    // No [TOOL] specified
    if tool_args_pos == 0 {
        log::trace!("No tool specified.");
        print_help();
        return Ok(());
    }

    match args[tool_args_pos].as_str() {

        "build" | "b"  => { log::trace!("Tool: BUILD"); return build(args[tool_args_pos..].to_vec()).map_err(|e| e.into()); },

        _ => { return Err(MainError::UnknownToolError{tool:args[tool_args_pos].to_owned()}); }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn build(args: Vec<String>) -> Result<(), parser::ParseError> {

  // if args.len() > 3 {
  //   error_too_many_args();
  //   return;
  // }

  let current_dir = env::current_dir().unwrap_or(std::path::PathBuf::new());
  let target_path = 
    if args.len() < 2 { current_dir } //std::path::PathBuf::from("demo/build.y") }
    else { 
      let args_path = std::path::PathBuf::from(args[1].as_str());
      if args_path.is_absolute() { args_path }
      else { current_dir.join(args_path) }
    };
    
  parser::parse(target_path)?;
  Ok(())
}
