#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // 내부 모두를 연관함수 
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    // 메서드가 아닌 연관 함수는 구조체의 새 인스턴스를 반환하는 생성자로 자주 활용
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}