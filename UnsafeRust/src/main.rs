fn main() {
    let mut num = 42;

    // Using unsafe block to dereference a raw pointer
    unsafe {
        let raw_ptr = &mut num as *mut i32;
        *raw_ptr += 10;
    }

    println!("Updated value: {}", num);
}
