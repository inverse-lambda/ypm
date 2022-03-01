use std::env;

mod parser;
mod util;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MainError {
    //#[error("Unknown tool name: {tool:}")]
    #[error("No such tool: `{tool:}`")]
    UnknownToolError {tool: String},
    //#[error(transparent)]
    //AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    ParseError(#[from] parser::ParseError)
}

// impl std::convert::From<anyhow::Error> for MainError {
//     fn from(err: anyhow::Error) -> Self {
//         MainError::AnyhowError(err)
//     }
// }


// impl From<anyhow::Error> for MainError {
//     fn from(err: anyhow::Error) -> MainError { MainError::AnyhowError(err) }
// }
// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

fn main() { // { //anyhow::Result<()> { //Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut tool_args_pos = 0;
    //let c = commandlines::Command::new();
    //let mut chosen_tool: Tool;

    // Logging
    //if args.contains("-v") || args.contains("--verbose")
    set_max_log_level(log::LevelFilter::Info);

    // // No arguments
    // if args.len() <= 1 {
    //     print_help();
    //     return Ok(());
    // }

    //let tool = args.iter().position(|a| !a.starts_with('-'));
    //log::info!("Tool Pos: {:?}, Args: {:?}",tool, args);

    // [OPTIONS] (several)
    for i in 1..args.len() { 
        match args[i].as_str() {
            "-v" | "--verbose"  => { set_max_log_level(log::LevelFilter::Debug); },
            "-vv"               => { set_max_log_level(log::LevelFilter::Trace); },
            _ if !args[i].starts_with('-') => { tool_args_pos = i; break; },
            _ => { }
        }
    }

    // No [TOOL] specified
    if tool_args_pos == 0 {
        print_help();
        return ();
    }

    // [TOOL] (only one)
    if let Err(e) = select_tool(args, tool_args_pos) {
        //eprintln!("{:#?}", e);
        log::info!("{:}", e.to_string());
        std::process::exit(1)
    }
}

fn select_tool(args: Vec<String>, tool_args_pos: usize) -> Result<(), MainError> { 
    match args[tool_args_pos].as_str() {

        "build" | "b"  => { return build(args[tool_args_pos..].to_vec()).map_err(|e| e.into()); },

        _ => { return Err(MainError::UnknownToolError{tool:args[tool_args_pos].to_owned()}); }
    }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Setting the maximum displayed logging level
 */
pub fn set_max_log_level(log_level: log::LevelFilter) {
    if let Err(e) = log::set_logger(&util::log::LOGGER)
        .map(|()| log::set_max_level(log_level)) {
        panic!("Unable to set log level to {}: {:?}", log_level, e)
    }
    log::debug!("Set log level: {}", log_level);
}

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
