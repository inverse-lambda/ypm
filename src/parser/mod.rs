//pub mod parse;

// use std::io::BufRead;
// use std::io::BufReader;
// use std::fs::File;
use thiserror::Error;
use std::path::PathBuf;
use y::lexer::Lexer;

//use std::env;
//use anyhow::{/*Context,*/ Result}; // with_context()
//use log::*;

#[derive(Debug, Error)]
pub enum ParseError {
    //#[error(transparent)]
    //AnyhowError(#[from] anyhow::Error),
    #[error("Error reading file: {file:?}")]
    FileReadError {file: String, error: std::io::Error},

    #[error("Error lexing file: {file:?}\n{error:?}")]
    LexingError {file: String, error: y::lexer::error::LexingError},
}


pub fn parse(path: PathBuf) -> Result<(), ParseError> {
    
    // TODO: Falls eine Datei (vs Directory)
    log::info!("Building {}", path.display());
    // if let Ok(contents) = std::fs::read_to_string(path) { // path moved
    //     let lexer = Lexer::new(contents);
    //     println!("Lexer: {:?}", lexer);
    // } else {    }
    
    // TODO: Wenn Directory!
    // TODO: Max Dateigröße checken...
    // DONE: Failed wenn kein gueltiges UTF-8!
    let content_res = std::fs::read_to_string(path.to_owned());
        //.with_context(|| format!("Could not read file `{}`.", path))?;
    if let Err(err) = content_res {
        //println!("Could not read file `{:?}`.", path);
        //return Err(anyhow::anyhow!(err));
        //anyhow::bail!("Error reading file: {:?}", err);
        return Err(ParseError::FileReadError{file: path.to_string_lossy().into_owned(), error: err});
    }

    let mut lexer = Lexer::new(content_res.unwrap());
    match lexer.lex() {
        Ok(v) => Ok(v),
        Err(e) => Err(ParseError::LexingError{file: path.to_string_lossy().into_owned(), error: e})
    }
        
    // return match std::fs::read_to_string(path) { // path moved
    //     Ok(contents) => {
    //         let mut lexer = Lexer::new(contents);
    //         //println!("Lexer: {:?}", lexer);
    //         lexer.lex()
    //     },
    //     Err(e) => {
    //         println!("({:?}) Error reading file: {}.", e.kind(), e);
    //         //panic!("Error reading file.");
    //         Ok(())
    //     },
    // };
}