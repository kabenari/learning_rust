//implementing on structs
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn area<V, W>(self, other: Rectangle<V, W>) -> Rectangle<T, W> {
        Rectangle {
            width: self.width,
            height: other.height,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect2 = Rectangle {
        width: "width",
        height: 'h',
    };

    let rect3 = rect1.area(rect2);

    println!(
        "rect3.width: {} , rect3.height : {}",
        rect3.width, rect3.height
    )
}
