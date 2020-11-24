// functions - store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Steve");

    // bind function values to variables
    let get_sum = add(5, 5);
    println!("sum: {}", get_sum);

    // closure
    let n3: i32 = 4;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("closure sum: {}", add_nums(3, 8));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
