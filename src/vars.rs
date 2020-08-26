// variables are immutable by default
// Rust is block-scoped

// VIDEO LEFT OFF AT 25:15

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    age = 38;

    println!("My name is {} and I'm {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}