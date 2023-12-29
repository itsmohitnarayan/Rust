fn main() {
    // Conditional statements (if, else)
    let number = 42;

    if number > 0 {
        println!("Number is positive");
    } else if number < 0 {
        println!("Number is negative");
    } else {
        println!("Number is zero");
    }

    // Using if in a let statement (ternary-like)
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("Number is {}", result);

    // Loops (for)
    println!("Printing numbers from 1 to 5 using a for loop:");
    for i in 1..=5 {
        println!("{}", i);
    }

    // Loops (while)
    println!("Printing numbers from 5 to 1 using a while loop:");
    let mut counter = 5;
    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }

    // Looping through an array using a for loop
    let numbers = [1, 2, 3, 4, 5];
    println!("Looping through an array:");
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Pattern matching with match
    let day = "Monday";

    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => {
            println!("It's a weekday!");
        }
        "Saturday" | "Sunday" => {
            println!("It's the weekend!");
        }
        _ => {
            println!("Invalid day");
        }
    }
}
