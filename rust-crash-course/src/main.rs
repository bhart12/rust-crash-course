#![deny(clippy::all)]

const MY_AGE: u8 = 25;

fn main() {
    let mut name: &str = "John";
    name = "Jane";
    
    let tuple_data = ("John", 22);

    let age: u8 = 20;
    let age2 = 20u8;
    let frac: f32 = 5.5;

    println!("Your name is {}", name);
    println!("Your tuple name is {}", tuple_data.0);
    println!("My const age {}", MY_AGE);
}
