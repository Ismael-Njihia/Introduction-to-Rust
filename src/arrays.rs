use std::mem;

pub fn run(){
    //Arrays - Fixed list where elements are the same data types
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers); 
    
    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //reassign value
    numbers[2] = 20;
    println!("Numbers: {:?}", numbers);
}