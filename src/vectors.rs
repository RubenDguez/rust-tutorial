use std::mem;

/** Arrays - Fixed list where elements are the same data types */

pub fn run() {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];
  println!("Numbers: {:?}", numbers);

  // Add onto vector
  numbers.push(5);
  numbers.push(6);

  println!("Push in values to the end of the vec with .push(val) : {:?}", numbers);

  // Pop off last value
  numbers.pop();

  println!("Pop off last value with .pop(): {:?}", numbers);

  // re-assign value
  numbers[3] = 20;
  println!("Numbers: {:?}", numbers);


  // get single value
  println!("First number: {}", numbers[0]);

  // length 
  println!("Vector length: {}", numbers.len());

  // Vector are stack allocated
  println!("Vector occupies {} bites", mem::size_of_val(&numbers));

  // get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // loop through vector values
  println!("Loop through vector values");
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // loop and mutate values
  println!("Loop and mutate values");
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Mutated values: {:?}", numbers);
}
