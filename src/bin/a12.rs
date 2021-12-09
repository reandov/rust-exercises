// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Brown,
    Orange,
    White,
    Black,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("color: black"),
            Color::Brown => println!("color: brown"),
            Color::Orange => println!("color: orange"),
            Color::White => println!("color: white"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct Box {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl Box {
    fn create_box(color: Color, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 2.0,
    };

    let small_box = Box::create_box(Color::Brown, 35.0, small_dimensions);
    small_box.print();
}
