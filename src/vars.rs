// Variables hold primitve data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);
    // can only do this is var is mutable
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars at once
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}