trait Printable {
    fn print(&self);
}

struct Rectangle{
    width: u32,
    height: u32,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle: {} x {}", self.width,self.height);

    }
}

fn main() {
    println!("Traits!");
    let Rect1 = Rectangle{width: 19, height: 12};
    Rect1.print();
}
