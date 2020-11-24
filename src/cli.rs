use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Jack";
    let status = "mostly";

    println!("command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how'reya?", name);
    } else if command == "status" {
        println!("{}", status);
    } else {
        println!("Not valid command.");
    }
}
