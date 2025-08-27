/**
 * Primitive str = Immutable fixed-length string somewhere in memory
 * String = Growable, heap-allocated data structure - Use when you need to modify or own string data
 */

pub fn run() {
  let mut hello = String::from("Hello ");

  
  // push char
  hello.push('W');
  
  // push_str
  hello.push_str("orld");
  
  // print the string literal
  println!("{}", hello);
  
  // get length
  println!("Length: {}", hello.len());
  
  // capacity
  println!("Capacity in bytes: {}", hello.capacity());

  // is empty
  println!("Is empty: {}", hello.is_empty());

  // contains substring
  let pattern = "Hello";
  let contains = hello.contains(pattern);
  println!("Contains {}: {}", pattern, contains);

  // replace
  println!("Replace: {}", hello.replace("World", "There"));

  // loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // create string with capacity
  let mut str_cap = String::with_capacity(10);
  str_cap.push('🦀');
  println!("str_cap: {}", str_cap);
  println!("str_cap Capacity: {}", str_cap.capacity());

  // Assertion testing
  assert_eq!(10, str_cap.capacity(), "We are testing the capacity of str_cap is {}", 10);
}
