fn main() {
    // Integers
    let x: i8 = 10;
    let y: i16 = 20;
    let z: i32 = 30;
    let w: i64 = 40;

    let a: u8 = 50;
    let b: u16 = 60;
    let c: u32 = 70;
    let d: u64 = 80;

    // Floating-point Numbers
    let f1: f32 = 3.14;
    let f2: f64 = 3.14159;

    // Booleans
    let is_true: bool = true;
    let is_false: bool = false;

    // Characters
    let char1: char = 'A';
    let char2: char = '😊';

    // Tuples
    let tuple: (i32, f64, char) = (42, 3.14, 'A');

    // Arrays
    let array: [i32; 3] = [1, 2, 3];

    // Strings
    let string: &str = "Hello, Rust!";

    // Option
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Print values
    println!("Signed Integers: {}, {}, {}, {}", x, y, z, w);
    println!("Unsigned Integers: {}, {}, {}, {}", a, b, c, d);
    println!("Floating-point Numbers: {}, {}", f1, f2);
    println!("Booleans: {}, {}", is_true, is_false);
    println!("Characters: {}, {}", char1, char2);
    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);
    println!("String: {}", string);
    println!("Some value: {:?}", some_value);
    println!("None value: {:?}", none_value);
}
