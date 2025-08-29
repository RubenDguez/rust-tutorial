use std::mem;

/** Arrays - Fixed list where elements are the same data types */

pub fn run() {
  let mut numbers: [i32; 5] = [1,2,3,4,5];
  println!("Numbers: {:?}", numbers);

  // re-assign value
  numbers[3] = 20;
  println!("Numbers: {:?}", numbers);

  // get single value
  println!("First number: {}", numbers[0]);

  // length 
  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bites", mem::size_of_val(&numbers));

  // get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}
