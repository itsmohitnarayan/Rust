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

    //Remove the value
    marks.remove("Java");
    println!("subjects remaining {}, remaining subjects are {:?}",marks.len(), marks);

    // loop through hashmap
    for (subject,mark) in &marks {
        println!("For {} you got  {} marks", subject,mark);
    }

    //check the value 
    print!("Did you studied C {} \n", marks.contains_key("c"));
    //check for another value
    print!("Did you studied Python {} ", marks.contains_key("Python"));

}
