//pub mod parse;

mod ast;

// use std::io::BufRead;
// use std::io::BufReader;
// use std::fs::File;
use std::{path::PathBuf, process::Command};
use thiserror::Error;
use y::lexer::Lexer;

//use std::env;
//use anyhow::{/*Context,*/ Result}; // with_context()
//use log::*;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum ParseError {
    //#[error(transparent)]
    //AnyhowError(#[from] anyhow::Error),
    #[error("Error reading file: {file:?}")]
    FileReadError { file: String, error: std::io::Error },

    #[error("Error lexing file: {file:?}\n{error:?}")]
    LexingError {
        file: String,
        error: y::lexer::error::LexingError,
    },
}
type Map<T> = HashMap<String, T>;
type Components = Map<Component>; // std::collections::HashMap<String, Component>;
type Methods = std::collections::HashMap<String, Method>;
type Params = HashMap<String, Param>;

struct Component {
    //name: The name/label itself is an external reference and might be varying
    components: Components,
    methods: Methods,
}
struct Method {
    params: Params,
}
struct Param {}

pub fn parse(path: PathBuf) -> Result<(), ParseError> {
    // TODO: Falls eine Datei (vs Directory)
    log::info!("Building {}", path.display());
    // if let Ok(contents) = std::fs::read_to_string(path) { // path moved
    //     let lexer = Lexer::new(contents);
    //     println!("Lexer: {:?}", lexer);
    // } else {    }

    file_to_ast(path)
}

fn file_to_ast(path: PathBuf) -> Result<(), ParseError> {
    // TODO: Wenn Directory!
    // TODO: Max Dateigröße checken...
    // DONE: Failed wenn kein gueltiges UTF-8!
    // File Read Error
    let content_res = std::fs::read_to_string(path.to_owned());
    //.with_context(|| format!("Could not read file `{}`.", path))?;
    if let Err(err) = content_res {
        //println!("Could not read file `{:?}`.", path);
        //return Err(anyhow::anyhow!(err));
        //anyhow::bail!("Error reading file: {:?}", err);
        return Err(ParseError::FileReadError {
            file: path.to_string_lossy().into_owned(),
            error: err,
        });
    }

    // Lexing Error
    let mut lexer = Lexer::new(content_res.unwrap());

    // if let Err(err) = lexed_items {
    //     return Err(ParseError::LexingError {
    //         file: path.to_string_lossy().into_owned(),
    //         error: err,
    //     });
    // }
    while let Some(item) = lexer.next_token() {}

    //let ast = ast::Ast::new(lexed_items.unwrap());

    //log::debug!("{:?}", lexed_items.as_ref());
    // match lexer.lex() {
    //     Ok(v) => Ok(v),
    //     Err(e) => Err(ParseError::LexingError{file: path.to_string_lossy().into_owned(), error: e})
    // }

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

    Ok(())
}
