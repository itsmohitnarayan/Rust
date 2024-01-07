
mod my_module {
    pub fn personal() {
        println!("Hello");
    }
}

fn main() {
    println!("Modules!");

    my_module::personal();
}
