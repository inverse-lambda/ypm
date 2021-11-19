// use num_bigint::BigInt;

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

