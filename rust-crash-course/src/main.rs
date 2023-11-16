#![deny(clippy::all)]

use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
//use std::cell::Cell;
use std::ops::AddAssign;
use futures::executor::block_on;
use tokio::time::{sleep, Duration};
use futures::Future;

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

//Function that calls another function
//fn process_name(name: &str, callback:fn(&str) -> ())
//{
//    callback(name);
//}

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

// This static tells Rust that the string slice needs to live throughout the life of the program, and not just until this function returns
fn get_full_name_ch_11() -> &'static str
{
    "John Doe"
}

// This says param "a" lives as long as the first variable that was passed in
// Same with "b" variable
// and the return value lives as long as the first variable that was passed in does
// NOTE that these parameters were created in main, so the return value lives that long
// fn get_random_name<'c, 'd>(a: &'c str, b: &'d str) -> &'c str
fn get_random_name<'l1>(a: &'l1 str, b: &'l1 str) -> &'l1 str
{
    a
}

// says str passed into name has to have same lifetime as Person_ch11 struct
struct Person_ch11<'a>
{
    name: &'a str,
}



// Chapter 12
#[derive(Debug)]
struct Person_ch12 {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName {
    fn full_name(&self) -> String;
}

impl HasFullName for Person_ch12 {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Can be used by anything that has the "HasFullName" trait shown above
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

// new function allows us to call Person_ch12::new() as shown in main
impl CanInitializeWithFullName for Person_ch12 {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person_ch12 {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 30,
        }
    }
}

impl fmt::Display for Person_ch12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} is {} years old", self.first_name, self.last_name, self.age)
    }
}


// Chapter 13
struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T>{
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

// Allows us to dereferce a value of our BoxedValue type with the *
// Requires std::ops::Deref
impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}



// Chapter 14
#[derive(Debug)]
struct Point_ch14<T> {
    x: T,
    y: T,
}

//AddAssign means the value has to be able to be added, like an int or float
impl<T> Point_ch14<T> {
    fn move_offset(&mut self, x: T, y: T)
    where
        T: AddAssign,
    {
        self.x += x;
        self.y += y;
    }
}

impl<T: PartialEq> PartialEq for Point_ch14<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}




// Chapter 16
async fn async_get_name() -> String
{
    "John".to_string()
}

async fn call_api_one() -> String
{
    sleep(Duration::from_secs(1));
    "One".to_string()
}

async fn call_api_two() -> String
{
    sleep(Duration::from_secs(1));
    "Two".to_string()
}

fn call_api_one_future() -> impl Future<Output = String>
{
    async
    {
        sleep(Duration::from_secs(1));
        "One".to_string()
    }
}

fn call_api_two_future() -> impl Future<Output = String>
{
    async
    {
        sleep(Duration::from_secs(1));
        "Two".to_string()
    }
}

// Allows main to become an async function
#[tokio::main]
async fn main() {
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



    // Chapter 10
    //value can be an &str or an error
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello, not an error");
    match value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    };

    let value: Result<&str, ()> = Err(());
    match value {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Some error occurred"),
    };

    // Did not write everything from ch 10



    // Chapter 11
    //&str (string slice) is a borrowed value
    // the string itself is being created somewhere, and we are just borrowing that value

    //String is an actual string object

    let full_name = get_full_name_ch_11();
    println!("{}", full_name);

    let name = get_random_name("john", "jane");
    println!("{}", name);

    //3 Lifetime rules
    //Rule 1: Compiler assigns lifetime operator to each parameter that is passed by referrence
    //Rule 2: If you have a single param that is a reference, and a single return value that is a reference, the compiler will automatically give the return value the same lifetime as that param
    //Rule 3: If you have a reference to &self or &mut, then the lifetime of self is assigned to return value



    // Chapter 12
    let person = Person_ch12 {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let person2 = Person_ch12::new("James Dough");
    println!("First name = {}, last name = {}, age = {}", person2.first_name, person2.last_name, person2.age);
    // Done with that fmt::Display line for Person_ch12
    println!("{}", person2);
    print_full_name_and_age(&person2);

    //Didn't write everything for this, but may go back



    //Chapter 13
    // Box stores values in the heap, while the pointer is on the stack, opposed to storing the value on the stack
    let age = Box::new(22);
    let twice = *age * 2;
    println!("{}", twice);

    let my_age = BoxedValue::new(22);
    println!("Value with MY deref is {}", *my_age);
    // In this case, *age = *(age.deref())

    //let array = vec!["John".to_string(), "Jane".to_string()];
    //let rc = Rc::new(array);
    //let weak = Rc::downgrade(&rc);
    //drop(rc);
    //weak.upgrade().unwrap();



    // Chapter 14
    let mut p1 = Point_ch14 {x:1.0, y:2.0};
    p1.move_offset(3.0, 4.0);
    println!("{:?}", p1);
    let mut p2 = Point_ch14 {x:3.0, y:4.0};
    // We can do this now because we implemented PartialEq for Point_ch14
    if p1 == p2
    {
        println!("p1 and p2 are equal");
    }
    else {
        println!("p1 and p2 are NOT equal");
    }



    // Chapter 15
    // block_on() is like a wait()
    let name = block_on(async_get_name());
    println!("Hello, {}!", name);

    // Tells fucntions to wait until they are complete
    let one = call_api_one().await;
    println!("{}!", one);
    let two = call_api_two().await;
    println!("{}!", two);

    let one = call_api_one_future().await;
    println!("{}!", one);
    let two = call_api_two_future().await;
    println!("{}!", two);

}
