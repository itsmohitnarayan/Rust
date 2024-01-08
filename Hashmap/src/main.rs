


use std::{collections::HashMap, hash::Hash}; 
fn main() {
    println!("Hashmap!");
    //declaring new hashmap as a marks variable
    let mut marks: HashMap<&str, i32> = HashMap::new();

    //adding values to the hashmap
    marks.insert("Rust", 80);
    marks.insert("Java", 50);
    marks.insert("Python", 90);
    marks.insert("Javascript", 70);
    marks.insert("C++", 90);
    marks.insert("React", 90);


    println!("{:?}", marks);

    //Find the length
    println!("How many subject you have Studied {}", marks.len());

    //Let match the value 
    match marks.get("React") {
        Some(mark) => println!("You Got {} in React", mark),
        None => println!("You haven't Studied this YET!")
    }

}
