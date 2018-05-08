// Allow us to format our Worker
#[derive(Debug)]
struct Worker {
    name: String,
    id: u64,
    task: String,
}

fn main() {
    // Remember w its not mutable. Name and task, receive a pointer to a
    // string, so if we loose that string or mix it in other scoope we
    // will receive an compilation error.
    let w = Worker{
        name: String::from("struct_exemple.rs"),
        id: 7,
        task: String::from("cargo run struct_exemple"),
    };

    // The way to print structs with macro print!.
    println!("This is your Worker: {:?}", w);
}
