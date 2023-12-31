// Defining a Struct
struct Person {
    name: String,
    age: u32,
}

// Instantiating Structs
fn instantiate_struct() {
    // Creating an instance of the Person struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Accessing fields of the struct
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}

// An Example Program Using Structs
fn example_program_using_structs() {
    // Creating instances of the Person struct
    let person1 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    let person2 = Person {
        name: String::from("Charlie"),
        age: 40,
    };

    // Printing information about the persons
    print_person_info(&person1);
    print_person_info(&person2);
}

// Function using a reference to a Person struct
fn print_person_info(person: &Person) {
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}

// Method Syntax
impl Person {
    // Method associated with the Person struct
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

// Method Syntax Example
fn method_syntax_example() {
    let person = Person {
        name: String::from("David"),
        age: 28,
    };

    // Calling the greet method using method syntax
    person.greet();
}

fn main() {
    // Instantiating Structs Example
    instantiate_struct();

    // Example Program Using Structs
    example_program_using_structs();

    // Method Syntax Example
    method_syntax_example();
}
