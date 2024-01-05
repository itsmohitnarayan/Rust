#[derive(Debug)]

enum Gender{
    Male,Female,Trans
}
#[derive(Debug)]
struct Person {
    name:String,
    email:String,
    age:i32,
    gender:Gender
}

fn main() {
    println!("Advance Enums!");

    let person1=Person{
        name:String::from("Mohit narayan"),
        email:String::from("mojsjfsfhfushfiue.com"),
        age:43,
        gender:Gender::Male
    };

    println!("{:?}",person1);

    let result = cal(3);
    println!("{:?}", result);
}

fn cal(no:i32) -> Option<bool>{
    if no % 2 == 0{
        Some(true)
    }else {
        None
    }
}
