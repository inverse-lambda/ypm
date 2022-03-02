//use anyhow::{Context, Result};
//use super::lexer::LexingError;
pub mod error;

use crate::lexer::error::LexingError;

type SourceSize = usize; // might also depend on maximum file size - Vec size is constrained/limited by usize
type LineNumber = u32;   // limits max number of allowed (source code) lines
type RowNumber  = u32;   // limits max number of characters per line
type IndentSize = RowNumber;   // limits max number of allowed indent space values


const END_OF_STRING: char = 0x00 as char;
//const INDENT_VALUE_INCREMENT_PER_SPACE: IndentSize = 1;
const INDENT_SPACE_INCREMENT_PER_TAB:   IndentSize = 4;
//const INDENT_FIXED_TAB_TO_SPACES_CONVERSION_LENGTH: IndentSize = 1; // ggfs auf Dauer nicht so simpel lassen?
const MAX_SOURCE_FILE_SIZE: u64 = 4294967295; // u32::MAX = 4294967295

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct TokenWithInfo {
    token: Token,
    line:  LineNumber,
    row:   RowNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Label { value: String },

    Equal,
    Indent { spaces:IndentSize },
}

impl TokenWithInfo {
    pub fn new(token: Token, line: LineNumber, row: RowNumber) -> Self {
        Self { token, line, row }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Lexer {
  //source: std::str::Chars<'a>, //Vec<char>,
  source:                     Vec<char>,
  position:                   SourceSize,
  position_of_last_linebreak: SourceSize,
  line_count:                 LineNumber,
  tokens:                     Vec<TokenWithInfo>
}

impl Lexer {

    pub fn new(source: String) -> Self { 
        Self { 
            source:                     source.chars().collect(), 
            position:                   0, 
            position_of_last_linebreak: 0,
            line_count:                 0,
            tokens:                     Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<(), LexingError> {

        if (usize::MAX as u64) < MAX_SOURCE_FILE_SIZE {
            println!("WARN: Actual max compilable source file size ({}) is smaller than general max source file size ({}).", usize::MAX, MAX_SOURCE_FILE_SIZE);
        }

        if (self.source.len() as u64) > MAX_SOURCE_FILE_SIZE {

            // return Err(std::io::Error::new(
            //     std::io::ErrorKind::InvalidInput, format!(
            //anyhow::bail!("Source length ({}) is bigger than maximum allowed ({}).", 
            return Err(LexingError::SourceTooBig{
                found: self.source.len(),
                max:   MAX_SOURCE_FILE_SIZE}
            );
        }

        while self.source.len() > self.position {
            let c = self.source.get(self.position).unwrap();
            //let num: u64 = c.to_string().parse().expect("not a number");
            //let next = self.peek_next(); // nicht generell nötig
            match c {
                //'#' => self.inline_comment(), // possibly annotations?
                '-' if self.peek_next()=='-' => self.inline_comment(),
                '/' if self.peek_next()=='/' => self.inline_comment(),
                //'/' if self.next_char()=='*' => self.comment_until("*/"),

                '=' => self.push_token(Token::Equal),
                
                '\n' => self.lex_linebreak_with_indent(),
                '\r' | ' ' | '\t' => {}, // self.igonre_whitespace(c)
                

                // Control characters
                _ if *c as u32 <= 0x1F => println!("Zeichen < 0x1F: {}", *c as u32),

                // Other identifier, type, lable, etc.
                _ => {}, // println!("{} - {}", c, *c as u32),
            }
            self.position += 1;
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////
    
    fn peek_next(&self) -> char {
        
        if self.position < self.source.len() - 1 { // this way around to not overflow when usize == u32 (with maxed out file size)
            *self.source.get(self.position+1).unwrap()            
        } else {
            println!("INFO: End of source during peek (pos = {}).", self.position);
            END_OF_STRING //0x00 as char
        }
    }

    fn until(&mut self, stop: char) -> String {
        let mut str: String = String::new();
        loop {
            let chr = *self.source.get(self.position).unwrap();
            if chr == stop || self.position+1 >= self.source.len() { return str }
            str.push(chr);
            self.position += 1;
        }
        
    }

    #[inline]
    fn get_current_row_number(&self) -> RowNumber {
        (self.position - self.position_of_last_linebreak) as RowNumber
    }

    fn push_token(&mut self, token: Token) {
        let line_number: LineNumber = self.line_count;
        let row_number: RowNumber = self.get_current_row_number();
        self.tokens.push(TokenWithInfo::new(token, line_number, row_number))
    }

    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    /// 
    fn inline_comment(&mut self) {
        let str = self.until('\n');
        log::trace!("Lex comment: {}", str)
    }

    fn lex_linebreak_with_indent(&mut self) {
        self.line_count += 1;
        self.position_of_last_linebreak = self.position;
        //let (mut tabs, mut spaces): (IndentSize, IndentSize) = (0, 0);
        let mut total_spaces: IndentSize = 0;
        let mut previous_spaces: IndentSize = 0;
        loop {
            match self.peek_next() {
                ' '  => { 
                    total_spaces += 1;
                    previous_spaces += 1;
                    // there might be a better - yet to develop - lambda-ish pattern for above increments
                },
                '\t' => {
                    // INFO: consume previous spaces that are not an exact multiple of the fixed tab size
                    total_spaces += INDENT_SPACE_INCREMENT_PER_TAB - (previous_spaces % INDENT_SPACE_INCREMENT_PER_TAB);
                    previous_spaces = 0;
                },
                _ => break,
            }
            self.position += 1;
        }
        log::trace!("Lex indent size: {}", total_spaces); // TODO: DEBUG LOG/LEVELS
        self.push_token(Token::Indent{spaces: total_spaces}); // INFO: rowNumber = letztes Zeichen, nicht erstes!!
    }
}