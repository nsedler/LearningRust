struct Rectangle {
    length: i32,
    width: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        &self.length * &self.width
    }
    fn perimeter(&self) -> i32 {
        2 * (&self.width + &self.length)
    }
}

fn main() {
    let rect = Rectangle {
        length: 10,
        width: 10
    };

    println!("{} units", rect.area());
    println!("{} units squared", rect.perimeter());
}
