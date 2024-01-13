#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn perimeter(&self) -> u32{
        2*(self.height + self.width)
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 20,
        height: 40
    };

    let sq = Rectangle::square(3);

    dbg!(&sq);

    println!("Rect = {:#?}", rect1);

    dbg!(&rect1);

    let area = calculate_area(&rect1);
    println!("Area of a rectangle of Area: {}", area);
    println!("The perimeter of the rectangle is: {}", rect1.perimeter());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    if rect1.can_hold(&rect2) {
        println!("Rect 1 holds Rect2");
    }
    
}

fn calculate_area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}
