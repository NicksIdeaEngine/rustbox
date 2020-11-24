// arrays - fixed list where elements are same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // reassign value
    numbers[2] = 20;
    // get full array
    println!("entire array: {:?}", numbers);
    // get single value
    println!("single value: {}", numbers[1]);
    // get array length
    println!("array length: {}", numbers.len());
    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));
    // get slice of array
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}
