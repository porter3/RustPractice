pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("{} is from {}", "Jake", "NJ");

    // positional args
    println!("{0} is from {1} and {0} likes to {2}", "Jake", "NJ", "code");

    // named args
    println!("{name} likes to play {activity}", name="John", activity="soccer");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}