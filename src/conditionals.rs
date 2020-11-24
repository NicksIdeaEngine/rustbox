// conditionals - check condition of something, act on results

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What'r ye 'avin?");
    } else if age < 21 && check_id {
        println!("Bartender: Y'ain't ol' 'nuf, boy!");
    } else {
        println!("Bartender: Show me yer card!");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age: {}", is_of_age);
}
