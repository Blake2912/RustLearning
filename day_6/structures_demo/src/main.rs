struct Rectangle(i32, i32);

fn main() {
    let rect1 = Rectangle(30,50);
    let len = rect1.0;
    let breadth = rect1.1;

    let area = calculate_area(rect1);
    println!("Area of the rectange length {len} and breadth {breadth} is {area}");
}

fn calculate_area(rect: Rectangle) -> i32{
    rect.0 * rect.1
}
