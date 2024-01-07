mod lib;


mod my_module {
    pub fn personal() {
        println!("Hello");
    }

}

mod my_mod2{
    pub mod movie{
        pub mod english{
            pub fn play(name:String){
                println!("Playing this movie {}", name);
            }
        }
        pub mod hindi{
            pub fn play1(name:String) {
                println!("Playing this movie {}",name);
            }
        }
    }
}

use my_mod2::movie::english::play;
use my_mod2::movie::hindi::play1;

fn main() {
    println!("Modules!");

    my_module::personal();

    lib::print_my_message();
    lib::print_my_message1();
    lib::print_my_message2();

    play("Iron man".to_string());
    play1("Swadesh".to_string());
}
