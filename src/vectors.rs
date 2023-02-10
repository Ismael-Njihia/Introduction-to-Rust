use std::mem;

pub fn run(){
    //Arrays - Fixed list where elements are the same data types
    //Vectors - Resizable list where elements are the same data types
    let mut numbers: Vec<i32>  = vec![1,2,3,4,5];
    println!("{:?}", numbers); 
    
    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get vECTOR length
    println!("VECTOR length: {}", numbers.len());

    //Vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //reassign value
    numbers[2] = 20;
    println!("Numbers: {:?}", numbers);

    //Add on to vector
    numbers.push(50);
    numbers.push(16);
    println!("Numbers: {:?}", numbers);

    //Pop off last value
    numbers.pop();
    println!("Numbers: {:?}", numbers);

    //lop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}