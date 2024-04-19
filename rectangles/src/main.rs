#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}



fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 52,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle::square(3);
    dbg!(rect3);

    println!("Area of rect is {} sq pixels", rect1.area());
    println!("Rectangle has non zero width:  {}", rect1.width());
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
}







// fn main() {
//     let rect1 = Rectangle {
//         width: 32,
//         height: dbg!(30*2),
//     };

//     println!("rect1 is {:#?}", dbg!(rect1))
// }

