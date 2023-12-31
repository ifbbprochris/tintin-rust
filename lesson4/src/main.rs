use std::ops::Add;

#[derive(Debug)]
enum TypeBox {
    Inter(i32),
    Text(String),
    Float(f64),
}

fn show_typebox(typeBox: &TypeBox) {
    match typeBox {
        TypeBox::Inter(i) => println!("inter: {i}"),
        TypeBox::Text(s) => println!("text: {s}"),
        TypeBox::Float(f) => println!("float: {f}"),
    }
}

fn test_case_one() {
    let vec = vec![
        TypeBox::Inter(100),
        TypeBox::Text("Hello world!".to_string()),
        TypeBox::Float(3.1415965),
    ];

    for v in vec {
        show_typebox(&v);
    }
}

trait TypeBoxTrait {
    fn show(&self);
}

impl TypeBoxTrait for i32 {
    fn show(&self) {
        println!("inter self fn: {}", *self);  
    }
}

impl TypeBoxTrait for String {
    fn show(&self) {
        println!("texttext self fn: {}", *self);  
    }
}

impl TypeBoxTrait for f64 {
    fn show(&self) {
        println!("float self fn: {}", *self);  
    }
}

fn call_show(x: &dyn TypeBoxTrait) {
    x.show();
}

fn test_case_two() {
    let vec:Vec<Box<dyn TypeBoxTrait>> = vec![
        Box::new(100),
        Box::new("Hello world!".to_string()),
        Box::new(3.1415965),
    ];

    for v in vec {
        call_show(&*v);
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other   .y,
        }
    }
}


fn main() {
    
    println!("test_case_one-------------------------------------------");
    test_case_one();
    println!("test_case_two-------------------------------------------");
    test_case_two();

    println!("test_add-------------------------------------------");

    let p1 = Point { x: 10, y: 20 };    
    let p2 = Point { x: 30, y: 40 };
    assert_eq!(p1 + p2, Point { x: 40, y: 60 });    
}
