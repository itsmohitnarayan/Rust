fn main() {
    println!("Tuples!");

    struct User (u8,u8,u8);

    let mut user1 = User(5,7,8);

    println!("{},{},{}",user1.0,user1.1,user1.2);

    user1.1 = 24;

    println!("{},{},{}",user1.0,user1.1,user1.2);


}
