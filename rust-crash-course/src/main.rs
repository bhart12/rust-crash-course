#![deny(clippy::all)]

use std::collections::HashMap;

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

#[derive(PartialEq)]
enum AnimalType {Dog, Cat, Rabbit}

enum Shapes {
    Circle { radius: f64, center: (f64, f64)},
    Rectangle { width: f64, height: f64, }
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



    // Chapter 7
    let fluffy = AnimalType::Dog;
    // switch statement
    match fluffy {
        AnimalType::Dog => println!("Woof!"),
        AnimalType::Cat => println!("Meow!"),
        AnimalType::Rabbit => println!("Hoot!"),
        _ => println!("Something else!")
    }


    let rectangle = Shapes::Rectangle { width: 3.0, height: 4.0 };

    if let Shapes::Rectangle { width, height } = rectangle
    {
        println!("width = {}, height = {}", width, height);
    }



    // Chapter 8
    let values = ("Hello", "World", 30);
    let vector: [&str; 2] = ["foo", "bar"];
    // will iterate through array and print each value on a new line
    for value in vector.iter()
    {
        println!("{}", value);
    }
    println!("Foo is {}", &vector[0]);
    let length = vector.len();

    // Create vector with unknown size
    let mut nums = vec![10, 20];
    nums.iter().map(|x: &i32| x * 2);
    nums.push(4);
    let four = nums.pop();
    println!("Nums are {:?}", nums);
    // add another vector onto your vector
    nums.extend_from_slice(&[4, 5, 6]);
    println!("Nums are {:?}", nums);
    // move contents from one array to another
    let mut values1 = vec![1, 2, 3];
    let mut values2 = vec![4, 5, 6];
    println!("values1 = {:?}", values1);
    println!("values2 = {:?}", values2);
    values1.append(&mut values2);
    println!("values1 = {:?}", values1);
    println!("values2 = {:?}", values2);

    if values1.contains(&3)
    {
        println!("value1 has 3");
    }

    let iter = nums.iter();
    let sum: i32 = iter.sum();
    println!("SUM is {}", sum);

    let mult_by_2: Vec<i32> = nums.iter().map(|v| v * 2).collect();
    println!("{:?}", mult_by_2);

    let names = vec!["John", "Jane", "Joe"];
    for name in names.into_iter().filter(|name| name.len() <= 3)
    {
        println!("{}", name);
    }

    // Be sure to include "use std::collections::HashMap;"
    let mut hash_values: HashMap<&str, &str> = HashMap::new();
    hash_values.insert("foo", "bar");

    for (&k, &v) in &hash_values
    {
        println!("{} {}", k, v);
    }

    if hash_values.contains_key("foo")
    {
        println!("Does contain foo");
    }
    hash_values.remove("foo");
    if !hash_values.contains_key("foo")
    {
        println!("Does NOT contain foo");
    }

    // will check if key "name" exists in hash map, and if not inserts one where the value is "Jane Doe"
    hash_values.entry("name").or_insert("Jane Doe");



    // Chapter 9
    //let name: Option<&str> = None;
    let name: Option<&str> = Some("John Doe");
    match name {
        Some(name) => println!("Hello {}!", name),
        None => println!("There is no name"),
    };

    //This will print name if it has a value, or the error message if name = None
    // this didn't work like he said it would
    //let unwrapped_name = name.expect("Name was not provided");
    //println!("Name is {}", unwrapped_name);

    let mut age: Option<i8> = Some(20);
    // this didn't work like he said it would
    //match age.as_mut() {
    //    Some(age) => *age += 10,
    //    None => {},
    //};
    println!("Age is {}", age.unwrap());

    // this didn't work like he said it would
    //let name2: Option<&str> = None;
    //let unwrapped = name2.unwrap_or("James Doe");
    //println!("name is {}", unwrapped);

    let name2: Option<&str> = None;
    if name2.is_some()
    {
        println!("There is a value");
    }
    else
    {
        println!("There is NOT a value");    
    }

}
