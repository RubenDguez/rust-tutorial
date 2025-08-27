// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ruben";
    println!("My name is {}", name);

    let mut age = 43;
    println!("My age was {} a year ago", age);

    age += 1;
    println!("My current age is {}", age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ruben", 43);
    println!("My name is {} and I am {} years old", my_name, my_age)
}
