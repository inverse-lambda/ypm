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

# Ideas 
- [ ] `*` and `**` for multiple param packing/unpacking (like Python, not C)
  - [ ] Unambiguous notation: `*` vs `[]` vs `-`
  - [ ] Dicts vs Lists / {} vs [] / named vs unnamed elements!!! (one of them as default?)
- [ ] `@` for sequential flow (also insert order relevant lists?????)


## Big Open Questions
Some of the main open decisions are:

- Python style indentation as block boundary vs new lines, braces, TOML style, etc.
  - [x] Indentation based, 4 indents default (same as even Google Code Guidlines for Python)
  - [ ] Tabs/Spaces Mixed, Tabs=4 Spaces? (+remove 1,2,3 previous spaces?)
    - [ ] Same + deeper level indents need to start the same way (seems intuitive)
      (1 Tab + 4 Spaces + 1 Tab -> next level allowed: 1 Tab + 4 Spaces + 2 Tabs?)????
      ((not practically ideal, if just 4 Tabs => Error))
    - [ ] Every level mixed, where 1 Tab = 4 Spaces (differently displayed when Tabs set to e.g. 2)

- Syntactic discrepancy between passing variables and declaring structure
- Syntactic discrepancy between colon (`:`) and equals (`=`)
  - possibly direct/initial/compile-time assigment with colon?
  - reference or both-sided continuous evaluation with equals?
- Syntactic discrepancy between referencing types and their instantiations
- Ambiguity of dot (`.`) syntax for `this` as well as member selector


## Package Management / Testing
- [?] No version range imports, only specific version(?) / Latest version???? (guarantees + inherited guarantees???)
- [x] Tests need "to be present" for any online repo/package
  - Tests are not allowed to be removed(! oO !) only added
  - Tests are permanent minimal "guarantees" of how a package will always behave (??)
- [ ] Version lock file
- [ ] Package Hashes instead of version numbers


## Reference Languages

- Squeak/Newsqueak/Go (Concurrency)
- Rust
- 