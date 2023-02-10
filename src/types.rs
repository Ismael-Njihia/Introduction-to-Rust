pub fn run(){
    let statement ="This is a type file!";
    println!("{}", statement);

     let x =10;
     let y =2.5;

     //Add explicit type
        let z: i64 = 454545454545454545;
    //find max size
        println!("Max i32: {}", std::i32::MAX);
        println!("Max i64: {}", std::i64::MAX);

    //Boolean
        let is_active = true;
    //Get boolean from expression
        let is_greater = 10 > 5;
        print!("{} is greater than 5: {}", x, is_greater);

    //Char
        let a1 = 'a';
        let face = '\u{1F600}';
        println!("{} {}", a1, face);

}