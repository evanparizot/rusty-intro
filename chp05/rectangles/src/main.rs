fn main() {
    // let w = 30;
    // let h = 50;
    // println!(
    //     "The are of the rectangle is {}", area(w,h)
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("{}", area(&rect1));
    println!("rect1 is {:?}", rect1);
    println!("rect1 area is {}", rect1.area());

    let sq = Rectangle::square(3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
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
            height: size
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }