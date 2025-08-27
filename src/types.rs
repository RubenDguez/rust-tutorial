/**
 * Primitive Types
 * Integers: u8, i8, u16, i16, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean: (bool)
 * Characters: (char)
 * Tuples
 * Arrays
 *
 * Rust is a statically typed language, which means that it must know the types of all
 * variables at compile time, however, the compiler can usually infer what type we want to use
 * based on the value and how we use it.
 */

pub fn run() {
    let x = 1; // by default (implied) this is "i32"
    let y = 2.5; // by default (implied) this is "f64"
    let z: i64 = 4646464646; // implicit type

    println!("x:{} y:{} z:{}", x, y, z);

    // Find max size
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    // Boolean
    let is_active = true;
    println!("Active user: {:?}", is_active);

    // get Boolean from expression
    let is_greater = 10 > 8;
    println!("10 > 8: {}", is_greater);

    let is_greater = 10 < 8;
    println!("10 < 8: {}", is_greater);

    // Char
    let a1 = 'a';
    let face: char = '😊';
    println!("I've got an {}, now my face is like {}", a1, face);
}
