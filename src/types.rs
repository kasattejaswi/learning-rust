// Integers, floats, bool, char, tuples, arrays
// Rust is statically typed language however compiler infers the data type

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 3.5;

    // Adding explicit type
    let z: i64 = 4545454544545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //  Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Char
    let a1 = 'a';
    let face = '\u{1f600}'; // Emoji unicode
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
