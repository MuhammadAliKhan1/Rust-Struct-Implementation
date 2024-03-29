#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };
    let square = Rectangle::square(5);
    println!("The area is {}", rect1.area());
    println!("The area is {}", square.area());
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Rect1 is {:#?}", rect1);
}
