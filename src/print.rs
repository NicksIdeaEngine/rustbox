pub fn run() {
    // print to console
    println!("Hello from print.rs!");
    // basic formatting
    println!("{} is from {}", "Nick", "Texas");
    // positional args
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Nick", "Texas", "code!"
    );
    // named args
    println!(
        "{name} likes to {activity}",
        name = "Nick",
        activity = "do yoga"
    );
    // placeholder traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);
    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
