fn main() {
    println!("Vector!");

    let mut abc = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", abc);

    abc.push(9);
    println!("{:?}", abc);

    // Insert the value 10 at index 2
    abc.insert(2, 10);
    println!("{:?}", abc);
}