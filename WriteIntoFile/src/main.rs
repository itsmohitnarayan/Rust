use std::fs::File;
use std::io::Write;



fn main() {
    println!("Lets write into file!");

    //let's create a new file
    let mut my_file = File::create("Output.txt").expect("could not able to create file");
    //write into file
    my_file.write_all(b"Welcome to the channel Mohit Narayan").expect("Not able to wrute into file");
    //append the data 
    my_file.write_all(b"Hey, I am Mohit").expect("Not able to write");
}
