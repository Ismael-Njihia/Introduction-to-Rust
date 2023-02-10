pub fn run(){
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is a {} student", "Ishmael", "KCA");

    // Positional Arguments
    println!("{0} is a {1} student and {0} is {2} years old", "Ishmael", "KCA", 21);
    
    // Named Arguments
    println!("{name} likes to play {activity}", name = "Ishmael", activity = "Football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}