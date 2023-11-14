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

fn say_hello_to_1(to_person: String) -> String
{
    format!("Hello, {}!", to_person)
}

fn process_name(name: &str, callback:fn(&str) -> ())
{
    callback(name);
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


    // Chapter 4
    let mut s1: String = String::from("James");
    // Create read only reference to s1
    let s2: &String = &s1;
    println!("s2: {}", s2);
    greet(&s1);
    greet(s2);
    empty_string(&mut s1);
    greet(&s1);


    // Chapter 5
    println!("{}", say_hello_to_1(String::from("Jerry")));
    // inline function
    let say_hello_to_full_name = |first_name: &str, last_name: &str| format!("Hello, {} {}!", first_name, last_name);
    println!("{}", say_hello_to_full_name("Jerry", "Johnson"));



}
