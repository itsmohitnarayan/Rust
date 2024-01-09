extern crate regex;

use regex::Regex;

fn main() {
    println!("Regex Example!");

    //ex1
    let Regex = Regex::new(r"w{6}").unwrap();

    let myname = "Mohit";

    println!("Found the correct: {}", re.is_match(myname));




}
