// vectors - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // reassign value
    numbers[2] = 20;
    // add on to vector
    numbers.push(5);
    numbers.push(6);
    // remove from end of vector
    numbers.pop();
    // get full vector
    println!("entire vector: {:?}", numbers);
    // get single value
    println!("single value: {}", numbers[1]);
    // get vector length
    println!("vector length: {}", numbers.len());
    // vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));
    // get slice of vector
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
    // loop through vector values
    for x in numbers.iter() {
        println!("number: {}", x);
    }
    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("numbers vec: {:?}", numbers);
}
