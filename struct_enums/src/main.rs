#[derive(Debug)] // formatting to {:?}
enum Status {
    Ready,
    // Block,
    Run,
    // Zombie,
}

#[derive(Debug)] // formatting to {:?}
struct Worker {
    name: String,
    id: u64,
    task: String,
    status: Status,
}

// This avoid us to declare Status in every single line, so its enough to
// say the type and we'll got our value enum.
// use Status::*; allow us to use every single value enum.
use Status::Ready;
use Status::Run;

fn main() {
    // A worker that contains a status field defined by our enum Status.
    // Now our worker is mutable because can change of status in its
    // lifetime.
    let mut w = Worker {
        name: String::from("struct_enums.rs"),
        id: 8,
        task: String::from("cargo run"),
        status: Ready,
    };
    println!("This is your Worker: {:?}", w);
    println!("Changing status to Run ...");
    w.status = Run;
    println!("This is your Worker: {:?}", w);
}
