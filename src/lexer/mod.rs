//use anyhow::{Context, Result};
//use super::lexer::LexingError;
pub mod error;

use crate::lexer::error::LexingError;

type SourceSize = usize; // might also depend on maximum file size - Vec size is constrained/limited by usize
type LineNumber = u32; // limits max number of allowed (source code) lines
type RowNumber = u32; // limits max number of characters per line
type IndentSize = RowNumber; // limits max number of allowed indent space values

//const ASSIGN_CHAR: char = ':';

const END_OF_STRING: char = 0x00 as char;
//const INDENT_VALUE_INCREMENT_PER_SPACE: IndentSize = 1;
const INDENT_SPACE_INCREMENT_PER_TAB: IndentSize = 4;
//const INDENT_FIXED_TAB_TO_SPACES_CONVERSION_LENGTH: IndentSize = 1; // ggfs auf Dauer nicht so simpel lassen?
const MAX_SOURCE_FILE_SIZE: u64 = 4294967295; // u32::MAX = 4294967295

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct TokenWithInfo {
    token: Token,
    line: LineNumber,
    row: RowNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Indent { spaces: IndentSize },
    Assign, // :

    Label { value: String },

    Equal,
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
    source: Vec<char>,
    position: SourceSize,
    position_of_last_linebreak: SourceSize,
    line_count: LineNumber,
    tokens: Vec<TokenWithInfo>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            position_of_last_linebreak: 0,
            line_count: 1, // line number count is not zero based
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<(), LexingError> {
        // Warn users on plattforms with smaller than 32bit usize (-> possibly rather return error and hinder build?)
        if (usize::MAX as u64) < MAX_SOURCE_FILE_SIZE {
            // TODO: Find better warning. Consider what else is affected from smaller usize (Vec size, etc.)
            log::warn!("Actual max compilable source file size ({}) is smaller than Inverse Lamda's max source file size ({}).", usize::MAX, MAX_SOURCE_FILE_SIZE);
        }

        // Error if source file is too big (4GB as of 2022)
        if (self.source.len() as u64) > MAX_SOURCE_FILE_SIZE {
            return Err(LexingError::SourceTooBig {
                found: self.source.len(),
                max: MAX_SOURCE_FILE_SIZE,
            });
        }

        self.lex_linebreak_and_indent();
        while self.source.len() > self.position {
            let c = self.source.get(self.position).unwrap();
            //let num: u64 = c.to_string().parse().expect("not a number");
            //let next = self.peek_next(); // nicht generell nötig
            match c {
                //'#' => self.inline_comment(), // possibly annotations?
                '-' if self.peek_next() == '-' => self.lex_comment_inline(),
                '/' if self.peek_next() == '/' => self.lex_comment_inline(),
                //'/' if self.next_char()=='*' => self.comment_until("*/"),
                '=' => self.push_token(Token::Equal),
                ':' => self.push_token(Token::Assign),

                '\n' => { self.lex_linebreak_and_indent(); continue; },
                '\r' | ' ' | '\t' => {} // self.igonre_whitespace(c)

                // Control characters
                _ if *c as u32 <= 0x1F => {
                    println!("Unexpected control character < 0x1F: {}", *c as u32)
                } // disallow / error?

                // Other identifier, type, lable, etc.
                _ => {
                    //let mut buffer: Vec<char> = Vec::<char>::new();
                    
                    let mut label: String = String::new();
                    loop {
                        if self.position >= self.source.len() {
                            break;
                        }
                        let ch = *self.source.get(self.position).unwrap(); //self.peek_next();
                        if ch==':' || ch=='\n' {
                            self.position -= 1;
                            break;
                        }
                        //label = concat_string!(label, ch);
                        label.push(ch);
                        self.position += 1;
                    }
                    log::trace!("Lex label: {}", label);

                    //println!("Lex unexpected: {}", c)
                    // println!("{} - {}", c, *c as u32),
                }
            }
            self.position += 1;
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////

    #[inline(always)] fn peek_next(&self) -> char {
        if self.position < self.source.len() - 1 {
            // this way around to not overflow when usize == u32 (with maxed out file size)
            *self.source.get(self.position + 1).unwrap()
        } else {
            // log::trace!("INFO: End of source during peek (pos = {}).", self.position);
            END_OF_STRING //0x00 as char
        }
    }

    // /**
    //  * Gives the character at position, and increments the position.
    //  * WARN: Does not check for bounds of `position > source.len()`!
    //  */
    // #[inline(always)] fn consume(&mut self) -> char { 
    //     let current_position = self.position;
    //     self.position += 1;
    //     *self.source.get(current_position).unwrap()
    // }

    #[inline(always)] fn current_char(&self) -> char { 
        if self.position >= self.source.len() { END_OF_STRING }
        else { *self.source.get(self.position).unwrap() }
    }

    /**
     * Gives the character at position, and increments the position.
     * INFO: Checks for source.len() and returns EOF (0x00) in case!
     */
    #[inline(always)] fn safe_consume(&mut self) -> char { 
        let current_position = self.position;
        self.position += 1;
        if current_position >= self.source.len() { END_OF_STRING }
        else { *self.source.get(current_position).unwrap() }
    }

    #[inline(always)] fn until(&mut self, stop: char) -> String {
        let mut str: String = String::new();
        loop {
            let chr = *self.source.get(self.position).unwrap();
            if chr == stop || self.position + 1 >= self.source.len() {
                return str;
            }
            str.push(chr);
            self.position += 1;
        }
    }

    #[inline(always)] fn get_current_row_number(&self) -> RowNumber {
        (self.position - self.position_of_last_linebreak) as RowNumber
    }

    #[inline(always)] fn push_token(&mut self, token: Token) {
        let line_number: LineNumber = self.line_count;
        let row_number: RowNumber = self.get_current_row_number();
        self.tokens
            .push(TokenWithInfo::new(token, line_number, row_number))
    }

    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////
    ///
    fn lex_comment_inline(&mut self) {
        let str = self.until('\n');
        // Set position back to last linebreak/EOF (except for an empty file)
        if self.position > 0 { 
            self.position -= 1; 
        }
        log::trace!("Lex comment: {}", str)
    }

    fn lex_linebreak_and_indent(&mut self) {
        //self.line_count += 1;
        //self.position_of_last_linebreak = self.position;
        //let (mut tabs, mut spaces): (IndentSize, IndentSize) = (0, 0);
        let mut total_spaces: IndentSize = 0;
        let mut previous_spaces: IndentSize = 0;
        loop {
            match self.current_char() { //self.safe_consume() { //self.peek_next() {
                ' ' => {
                    total_spaces += 1;
                    previous_spaces += 1;
                    // there might be a better - yet to develop - lambda-ish pattern for above increments
                },
                '\t' => {
                    // INFO: consume previous spaces that are not an exact multiple of the fixed tab size
                    total_spaces += INDENT_SPACE_INCREMENT_PER_TAB
                        - (previous_spaces % INDENT_SPACE_INCREMENT_PER_TAB);
                    previous_spaces = 0;
                },
                '\r' => {}, // ignore carriage returns in itself
                '\n' => {   // line-feed before any data => next
                    self.line_count += 1;
                    self.position_of_last_linebreak = self.position;
                    total_spaces = 0;
                    previous_spaces = 0;
                },
                END_OF_STRING => return, // nothing left to track
                _ => { /* self.position = 
                    if self.position > 1 { self.position-2 } 
                    else { 0 }; */
                    log::trace!("New content at line: {}", self.line_count);
                    break; 
                },
            }
            self.position += 1;
        }
        log::trace!("Lex indent size: {}", total_spaces); // TODO: DEBUG LOG/LEVELS
        self.push_token(Token::Indent {
            spaces: total_spaces,
        }); // INFO: rowNumber = letztes Zeichen, nicht erstes!!
    }
}
