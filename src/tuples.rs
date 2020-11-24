// tuples group together valeus of different types
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Nick", "Texas", 31);

    println!(
        "{} is from {} and is {} years old.",
        person.0, person.1, person.2
    );
}
