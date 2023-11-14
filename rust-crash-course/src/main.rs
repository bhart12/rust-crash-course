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

struct Person {
    name: String,
    age: u8,
}

fn create_person(name: String, age: u8)
{
    // Can use field init short hand if variable has same name as struct field
    let person: Person = Person {
        name,
        age
    };
}

#[derive(Debug)]
//this is a tuple (NOTE that this uses () instead of {} like a normal struct)
struct Point(f64, f64, f64);

impl Point
{
    fn describe(&self)
    {
        println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
    }
    fn zero() -> Point
    {
        Point(0.0, 0.0, 0.0)
    }
}

impl Point
{
    fn make_twice(&mut self)
    {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    fn twice(&self) -> Point
    {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
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


    // Chapter 6
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    println!("{} is {} years old", person.name, person.age);
    // bring over all fields of person variable, except specified fields
    let person2 = Person {
        name: "Doe".to_string(),
        ..person
    };
    println!("{} is {} years old", person2.name, person2.age);

    let point = Point(0.0, 1.0, 2.0);
    println!("x = {}, y = {}, z = {}", point.0, point.1, point.2);
    point.describe();
    println!("{:?}", point);

    let mut p = Point(1.0, 2.0, 3.0);
    println!("{:?}", p);
    let twice = p.twice();
    println!("{:?}", twice);
    let p0 = Point::zero();
    println!("{:?}", p0);

}
