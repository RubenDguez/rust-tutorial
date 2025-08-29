mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;

fn main() {
    println!("=== PRINT ===");
    print::run();

    println!("\n=== VARS ===");
    vars::run();

    println!("\n=== TYPES ===");
    types::run();

    println!("\n=== STRINGS ===");
    strings::run();

    println!("\n=== TUPLES ===");
    tuples::run();

    println!("\n=== ARRAYS ===");
    arrays::run();

    println!("\n=== VECTORS ===");
    vectors::run();

    println!("\n=== Conditionals ===");
    conditionals::run();
}
