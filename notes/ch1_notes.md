https://doc.rust-lang.org/book

# Chapter 1

### 1.1 Hello World

main.rs (.rs is rust extension)
fn main() {
    println!("Hello, world!");
}

rust files are compiled, then run:
$ rustc main.rs
$ ./main
Hello, world!

prinln! <- the ! is for macro calls as opposed to functions

rust uses semicolons

### 1.2 Cargo

Cargo is the dependency manager used for rust. uses TOML format
[package] - section for typical package information
[dependencies] - self explanatory section, packages are called "crates"

Cargo, like most standards, expects your source files to live inside the src directory
Cargo manages a cargo.lock, which never needs manual update

Cargo build:
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
$ ./target/debug/hello_cargo 
Hello, world!

"cargo run" combines the above 2 commands 
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!

Cargo only recomiles code when there is a file change
"cargo check" will compile without producing an executable, useful for checking if code will compile

"cargo build --release" will perform compiler optimizations and create an executable in target/release

$ cargo new project_name
Creates new Cargo.toml and hello world /src/ structure
IntelliJ rust support automatically sets up all the structure and code of chapter1 through "cargo new"

