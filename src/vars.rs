pub fn run(){
    let name = "Ishmael";

    let mut age = 21;
    println!("My name is {} and I am {}", name, age);
    age = 22;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32= 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ishmael", 21);
    println!("{} is {}", my_name, my_age);
}