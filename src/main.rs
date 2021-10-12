use std::env;
mod parser;

fn print_help() {
	println!("
Inverse Lambda Manager
----------------------

USAGE:  
        y [TOOLS] [OPTIONS]

TOOLS:
        build, b        Compile the current package
        
OPTIONS:
        -h, --help      Prints help information
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
		
		"build" | "b"  => {
			let current_dir = env::current_dir()?;
			let target_path = std::path::PathBuf::from("demo/build.y");
			parser::parse(target_path, current_dir)
		}
		
		_ => { print_help(); }
	}
	Ok(())
}

