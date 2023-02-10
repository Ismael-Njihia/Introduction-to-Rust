
struct Color{
    red: u8,
    green: u8,
    blue: u8
}
//Tuple start
struct Color2(u8, u8, u8);

struct  Person{
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    } 

    //Get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //sety last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }
    //name to tuple

    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}



pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    
    let mut c2 = Color2(255, 0, 0);
    c2.0 = 200;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Ishmael", "Njihia");
    
    p.set_last_name("Nganga");
    println!("person {}", p.full_name());

    println!("person {:?}", p.to_tuple());
}