fn main() {
    println!("Stack, Heap!");
    stack_mem_ex();
    heap_mem_ex();
}
// Stack Example
fn stack_mem_ex(){
    let x =9;
    let y =8;

    let sum = add(x,y);

    println!("The sum of x{} and y {} is {}",x,y,sum);

    fn add(a: i32, b: i32) -> i32{
        a+b
    }
}
// Heap Example
fn heap_mem_ex(){
    let mut V = Vec::new();//dynamically allocated memory for vector v
    V.push(1);
    V.push(11);
    V.push(111);
    V.push(11111);
    println!("{:?}", V);
}