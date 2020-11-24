// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Nick";
    let mut age = 31;
    println!("My name is {} and I am {} years old.", name, age);
    age = 32;
    println!("My name is {} and I am {} years old.", name, age);
    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);
    // assign multiple variables
    let (my_name, my_age) = ("Nick", 31);
    println!("{} is {}", my_name, my_age);
}
