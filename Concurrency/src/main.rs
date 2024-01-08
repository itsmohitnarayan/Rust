//importing some packages
use std::thread;
use std::time::Duration;




fn main() {
    println!("Concurrency!");
    // create a new thread to run parallel with main function
    thread::spawn(||{
        for i in 1..10 {
            println!("The numbers are {} from the spawned thread! \n", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //code run from main function
    for i in 1..5 {
        print!("The number are {} from the main thread \n",i);
        thread::sleep(Duration::from_millis(1));
    }
}
