//! Factory method creational design pattern allows creating objects without having to specify the exact type of the object that will be created.

trait Shape {
    fn draw(&self);
}

enum ShapeType {
    Rectangle,
    Circle,
}

struct Rectangle{}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing a Rectangle");
    }
}

struct Circle{}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

struct ShapeFactory {}
impl ShapeFactory {
    fn new_shape(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle {}),
            ShapeType::Rectangle => Box::new(Rectangle {}),
        }
    }
}

fn main() {
    let shape = ShapeFactory::new_shape(&ShapeType::Circle);
    shape.draw();

    let shape = ShapeFactory::new_shape(&ShapeType::Rectangle);
    shape.draw();
}
