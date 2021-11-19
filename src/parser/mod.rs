//pub mod parse;

// use std::io::BufRead;
// use std::io::BufReader;
// use std::fs::File;
use std::path::PathBuf;
//use std::env;

pub fn parse(file_path: PathBuf, current_wdir: PathBuf) {
    // Given file_path absolute, then use it, otherwise join with working directory path
    let path = 
        if file_path.is_absolute() { file_path }
        else { current_wdir.join( file_path ) };
    println!("TODO: build  {}", path.display());
}