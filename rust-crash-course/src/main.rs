#![deny(clippy::all)]

const MY_AGE: u8 = 25;

// Print string passed by reference
fn greet(name: &String)
{
    println!("Hello {}!", name);
}

// Alter string passed by reference. MUST BE mutable
fn empty_string(value: &mut String)
{
    value.clear();
}

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

    let mut s1: String = String::from("James");
    // Create read only reference to s1
    let s2: &String = &s1;
    println!("s2: {}", s2);
    greet(&s1);
    greet(s2);
    empty_string(&mut s1);
    greet(&s1);
}
