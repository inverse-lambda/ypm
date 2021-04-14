use std::env;

fn help() {
	println!("Inverse Lambda's manager

USAGE: 
	y [TOOLS] [OPTIONS]

OPTIONS:
	-h, --help				Prints help information
	
TOOLS:
	build, b				Compile or transpile the current package
	
");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	// no arguments passed
	if args.len() <= 1 {
		help();
		return;
	}
	
	// any other number of arguments
	match args[1].as_str() {
		
		"build" | "b"  => {
			println!("TODO: build!");
		}
		
		_ => { help(); }
	}
}
