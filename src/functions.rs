pub fn run(){
    greeting("Ishmael");

   // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(3, 3));
}
fn greeting(name: &str) {
    println!("Hello {}", name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}