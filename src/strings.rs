pub fn run(){
    let mut hello = String::from("Hello");

    

    hello.push('W');
   

    //push a string
    hello.push_str("orld!");
    print!("{}", hello);

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is Empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'World' {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //assertion testing
    assert_eq!(1, s.len());
    assert_eq!(10, s.capacity());


    //get length
    println!("Length: {}", hello.len());

}