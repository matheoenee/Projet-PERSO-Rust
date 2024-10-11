// From and Into
// The From and Into traits are inherently linked, and this is actually part of its implementation. 
// If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.

use std::convert::From;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

/* 
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
*/

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

// ToString

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // let num = Number::from(30);
    // println!("My number is {:?}", num);

    let int = 5;
    let num2: Number = int.into();
    println!("My number 2 is {:?}", num2);

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}