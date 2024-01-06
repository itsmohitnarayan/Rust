struct Rectangle{
    height : u32,
    width : u32,
}

impl Rectangle {
    fn printdescription(&self) {
        println!("Rectangle width is {} and height is {}", self.width, self.height);
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    fn cal_square(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    
    let my_rectangle = Rectangle{width:45,height:45};
    my_rectangle.printdescription();
    println!("Rectangle is square:!{}", my_rectangle.is_square());
    println!("Square area of rectangle is {}", my_rectangle.cal_square());
    
}
