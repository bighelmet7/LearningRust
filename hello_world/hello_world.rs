// Hello World

// println!() its a macro, that means is doing some Rust metaprogramming.
// A macro in Rust:
// - implicitly takes its arguments by reference even when 
// they're passed by value.
// - they can take variable number of arguments, a function has a fixed
// number of arguments.
//
// See Rust Cargo.

fn main() {
    println!("Hello Rust!");
}
