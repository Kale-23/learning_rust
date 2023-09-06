struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_fit(&self, r: Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }

    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 10,
        height: 30,
    };

    let r3 = Rectangle::square(30);

    println!("Fits? {}", r1.can_fit(r2));

} 