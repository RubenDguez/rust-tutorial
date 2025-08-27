pub fn run() {
    println!("Hello from print rs");
    println!("Number: {}", 1);

    // basic formatting
    println!("{} is from {}", "Ruben", "FL");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ruben", "Florida", "Code"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // placeholder traits
    println!("Binary: {:b}; Hex: {:x}; Octal: {:o}", 10, 10, 10);

    // since in this example we are using the same value throughout the print statement
    // we can use Positional Argument to reference them as bellow
    println!("Binary: {0:b}; Hex: {0:x}; Octal: {0:o}", 10);

    // debug traits
    println!("{:?}", (12, true, "Hello")); // using a tuple

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
