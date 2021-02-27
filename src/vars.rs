// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Tejaswi";
    // To make var mutable, assign mut to it
    let mut age = 22;
    println!("My age is {}", age);
    age = 23;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Tejaswi", 22);
    println!("My name is {} and  my age is {}", my_name, my_age);
}
