extern crate rand;

//This is the way how rust import packages
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let da game begin!");
    println!("Gues the number, please introduce a number");

    let secret = rand::thread_rng().gen_range(1, 101);

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
    
    println!("And the secret is ... {}", secret);

    //We're shadowing the guess variable, so we can
    //have the same type as secret, two integers.
    //The ':' is telling Rust that we are annotate the
    //type of the variable guess in this case a unsigned integer.  

    let guess : u32 = guess.trim().parse()
        .expect("Damn it need a number m8!");

    // match is almost as a switch statement:
    // match variable {
    //  case => result,
    //  _    => println!("this is a default"),
    // }
    match guess.cmp(&secret) {
        Ordering::Less => println!("Muu peque"),
        Ordering::Equal => println!("Igualito illo!"),
        Ordering::Greater => println!("Teee pasaste la wea!"),
    } 
}
