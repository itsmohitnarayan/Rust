fn main() {
    println!("Reference!");

    let x = 90;

    let y =&x;

    let z =&y;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
}
