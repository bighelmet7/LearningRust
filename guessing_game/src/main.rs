//This is the way how rust import packages
use std::io;

fn main() {
    println!("Let da game begin!");
    println!("Gues the number, please introduce a number");

    // guess is a mutable variable, in Rust there are two ways
    // to declare a let variable, the default one:
    // let bar = 1;
    // this mean bar cant change its value.
    // And to make it mutable:
    // let mut bar = 1; //now it can changes it value
    // bar = 2;
    // But there's a way to change a value of a let, its call shadow
    // we can shadow let bar = 1; in this way:
    // let bar = 2; at this point bar its shadow and change it value.
    // The main diff between let mut, is that every time you call let
    // over the same variable, you're creating this variable every time
    // you call it.

    let mut guess = String::new(); // creates a new instance.

    io::stdin().read_line(&mut guess)
        .expect("Fail while reading..."); //Exception Err

    println!("Duuude! you gues {}", guess); //Text formating
}
