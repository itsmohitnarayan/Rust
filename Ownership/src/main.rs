// Ownership in Rust
fn ownership_example() {
    // Creating a String
    let original_string = String::from("hello");

    // Passing ownership to a function
    let length = calculate_length(original_string);

    // Attempting to use original_string here would result in a compilation error

    // Print the result
    println!("Length of the string: {}", length);
}

// Function taking ownership of a String
fn calculate_length(s: String) -> usize {
    let length = s.len();
    length
} // s goes out of scope, and its memory is freed

// References and Borrowing
fn references_and_borrowing() {
    // Creating a String
    let original_string = String::from("hello");

    // Borrowing a reference to the String
    let length = calculate_length_borrowing(&original_string);

    // Using original_string is still valid here

    // Print the result
    println!("Length of the string: {}", length);
}

// Function borrowing a reference to a String
fn calculate_length_borrowing(s: &String) -> usize {
    let length = s.len();
    length
} // s is a borrowed reference, and it does not own the String

// The Slice Type
fn slice_example() {
    // Creating a String
    let s = String::from("hello");

    // Creating a slice from the String
    let slice = &s[0..3];

    // Print the slice
    println!("Slice: {}", slice);
}

fn main() {
    // Ownership Example
    ownership_example();

    // References and Borrowing Example
    references_and_borrowing();

    // Slice Type Example
    slice_example();
}
