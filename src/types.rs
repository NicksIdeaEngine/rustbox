/* primitive types::
 * integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (num of bits they take in memory)
 * floats: f32, f64
 * boolean (bool)
 * characters (char)
 * tuples
 * arrays
 * */

// rust is statically typed, meaning it must know the types of all vars at compile time
// the compiler can usually infer intended type based on value and how it is used

pub fn run() {
    // int default is 'i32'
    let x = 1;
    // float default is 'f64'
    let y = 2.5;
    // add explicit type
    let z: i64 = 452309823087092;
    // find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);
    println!("max i128: {}", std::i128::MAX);
    // boolean
    let is_active: bool = true;
    // get bool from expression
    let is_greater: bool = 10 > 5;
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
