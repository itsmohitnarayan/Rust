use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("ReadFile!");

    //Open the file
    let mut file = File::open(r"C:\Users\Mohit\Rust\ReadFile\src\intro.txt").expect("Not able to find the file");

    // Read the File
    let mut content = String::new();
    //Process the File 
    file.read_to_string(&mut content).expect("Not able to read the file");

    println!("File contains this data {} \n", content);
}
