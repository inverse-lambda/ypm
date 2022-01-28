use std::env;
mod parser;

fn print_help() {
  println!("
Inverse Lambda Manager
----------------------

USAGE:  
        y [TOOLS | OPTIONS]

TOOLS:
        build, b                    Compile the current package
        build, b  [PATH]            Compile the target package
        
OPTIONS:
        -h, --help                  Prints this information
");
}

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  
  // No arguments
  if args.len() <= 1 {
    print_help();
    return Ok(());
  }
  
  // Some argument(s)
  match args[1].as_str() {
    
    "build" | "b"  => { build(args) }
    
    _ => { print_help(); Ok(()) }
  }
}

pub fn build(args: Vec<String>) -> std::io::Result<()> {

  // if args.len() > 3 {
  //   error_too_many_args();
  //   return;
  // }

  let current_dir = env::current_dir()?;
  let target_path = 
    if args.len() <= 2 { current_dir } //std::path::PathBuf::from("demo/build.y") }
    else { 
      let specified = std::path::PathBuf::from(args[2].as_str());
      if specified.is_absolute() { specified }
      else { current_dir.join(specified) }
    };
    
  parser::parse(target_path)
}
