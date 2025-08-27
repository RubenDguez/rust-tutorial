/**
 * Tuples group together values of different types
 * Max 12 elements
 */

pub fn run() {
  let person: (&str, &str, u8) = ("Ruben", "Florida", 44);

  println!("{} is from {}, and {} is {} years old.", person.0, person.1, person.0, person.2);
}
