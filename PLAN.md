# Development Plan (for Inverse Lambda PM)
A build system and package manager for the Inverse Lambda (y?) programming language.

## Overview
- [x] Basically specify language goals and syntax
- [ ] Develop parser
  - [ ] Develop a coarse lexer
    - [ ] Parse "semicolon"
    - [ ] Parse LVALUES
    - [ ] Parse types
    - [ ] Parse https references
  - [ ] Develop basic AST
  - [ ] Parse first tokens
- [ ] Refine language specification according to experience from lexer/parser development

## Big Open Questions
Some of the main open decisions are:

- Python style indentation as block boundary vs new lines, braces, TOML style, etc.
  (Possibly, the indentation level mostly for variable+component scope/visibility, not mainly for functionality relevant ???????)
- Syntactic discrepancy between passing variables and declaring structure
- Syntactic discrepancy between colon (`:`) and equals (`=`)
  - possibly direct/initial/compile-time assigment with colon?
  - reference or both-sided continuous evaluation with equals?
- Syntactic discrepancy between referencing types and their instantiations
- Ambiguity of dot (`.`) syntax for `this` as well as member selector