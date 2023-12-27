fn main() {
    // Immutable Variable
    let x = 5;
    println!("Immutable variable x: {}", x);

    // Mutability: Use 'mut' keyword to make a variable mutable
    let mut y = 10;
    println!("Mutable variable y (before mutation): {}", y);

    // Mutate the variable
    y = 15;
    println!("Mutable variable y (after mutation): {}", y);

    // Shadowing: Declare a new variable with the same name
    let z = "initial";
    println!("Variable z (before shadowing): {}", z);

    let z = "shadowed";
    println!("Variable z (after shadowing): {}", z);

    // Shadowing with different data types
    let a = 5;
    println!("Variable a (before shadowing): {}", a);

    let a = "shadowed";
    println!("Variable a (after shadowing): {}", a);

    // Shadowing with mutable variable
    let mut b = 3;
    println!("Mutable variable b (before shadowing): {}", b);

    let b = b * 2; // Shadowing and creating a new immutable variable
    println!("Variable b (after shadowing): {}", b);
}
