# Chapter 2

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
Brings "io" library into scope
"prelude" libraries which are in scope for every program by default -> https://doc.rust-lang.org/std/prelude/index.html

"let" creates a variable. Variables are immutable my default. "let mut" creates a mutable variable

String::new <- "new" is an "associated method" (static method in other languages)

.read_line(&mut guess) <- & indicates the argument is a reference, and "mut" means the function can modify it
read_line returns an enum Result with variants "Ok" and "Err". 
"expect" crashes the program on "Err", and returns the "Ok" value on "Ok". 
Not including .expect() will cause compile warning

semi-colons allows for easy multi-line expressions

let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);

Add new dependencies to Cargo.toml:
rand = "0.5.5"
lib = "version"

Cargo only downloads new/updated dependencies on "cargo build"

$ cargo doc --open
Opens all crate documentation in browser

"match" expression made of "arms" which have a "pattern" for matching

Rust has strong static types, and also type inference
Rust allows "shadowing" (or reusing/overwritting) variable names

"trim" eliminates whitespace
"parse" converts String to other type
"break" exits "loop", "continue" skips the remaining current loop and goes straight to the next

