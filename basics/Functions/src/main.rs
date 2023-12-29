// Function with no parameters and no return value
fn greet() {
    println!("Hello, world!");
}

// Function with parameters and a return value
fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum // Implicit return without a semicolon
}

// Function with a named return value
fn multiply_numbers(a: i32, b: i32) -> i32 {
    let product = a * b;
    return product; // Explicit return with a semicolon
}

// Function with a default parameter value
fn greet_person(name: &str, greeting: &str) {
    println!("{} {}", greeting, name);
}

// Function with a variable number of arguments
fn print_numbers(numbers: &[i32]) {
    for &num in numbers {
        println!("Number: {}", num);
    }
}

// Main function
fn main() {
    // Call the greet function
    greet();

    // Call the add_numbers function
    let result1 = add_numbers(5, 7);
    println!("Sum: {}", result1);

    // Call the multiply_numbers function
    let result2 = multiply_numbers(3, 4);
    println!("Product: {}", result2);

    // Call the greet_person function
    greet_person("Alice", "Hi");

    // Call the print_numbers function with a slice
    let numbers = [1, 2, 3, 4, 5];
    print_numbers(&numbers);
}
