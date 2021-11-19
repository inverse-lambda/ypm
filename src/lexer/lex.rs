// use num_bigint::BigInt;
use std::char;

pub enum NumType {
    Float,
    Complex, // real + imag (f64 + f64), necessary at this point??
    Int,
    Hex,
    Oct,
    Bin,
}

// Comments are not at all tokenized [//, /* */, #, etc.]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {

    Label { value: String },
    Colon,

    String { value: String }, // "quoted" string literal
    Number { representation: String, numtype: NumType },

    Indent,
    Dedent,

    EoF,
}

pub struct Lexer<T: Iterator<Item = char>> {
    iter: T,
    // indentation: Vec<IndentationLevel>,
    chr0: Option<char>,
    chr1: Option<char>,
}

impl<T> Lexer<T> where 
    T: Iterator<Item = char>,
{
    pub fn new(input_iterator: T, start: Location) -> Self {
        let mut lex = Lexer {
            chr0: None,
            chr1: None,
            // indentation: vec![Default::default()],
            iter: input_iterator,
        }
    }
}