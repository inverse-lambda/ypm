//pub mod parse;

// use std::io::BufRead;
// use std::io::BufReader;
// use std::fs::File;
use std::path::PathBuf;
//use std::env;

pub fn parse(file_path: PathBuf, starting_dir: PathBuf) {
    let path = 
        if file_path.is_absolute() { file_path }
        else { starting_dir.join( file_path ) };
    println!("TODO: build  {}", path.display());
}