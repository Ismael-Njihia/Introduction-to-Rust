pub fn run(){
    //A tuple is a collection of values of different types
    let person: (&str, &str, i8) = ("Ishmael", "KCA", 21);
    println!("{} is a {} student and is {} years old", person.0, person.1, person.2);

}