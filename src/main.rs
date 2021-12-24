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
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {:?}", rect1);

    println!("area of rectangle: {} square pixel", rect1.area());

    // can_hold
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect2 is contained to rect1? {}", rect1.can_hold(&rect2));
    println!("rect3 is contained to rect1? {}", rect1.can_hold(&rect3));
}
