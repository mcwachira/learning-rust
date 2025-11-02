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

#[derive(Debug)]
enum Color{
    Red,
}

impl Color {
    fn print(&self){
        match self {
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions{
    height:i32,
    length:i32,
    width:i32,
}

struct ShippingBox{
    dimensions:Dimensions,
    weight:f64,
    color:Color,
}

impl Dimensions {
    fn print(&self){
        println!("{:?} Box Height", self.height);
        println!("{:?} Box Width", self.width);
        println!("{:?} Box Length", self.length);
    }
}

impl ShippingBox {

    fn new (dimensions:Dimensions,
            weight:f64,
            color:Color) -> Self{
        Self{   dimensions,
                weight,
                color}
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("{:?} Box Weight", self.weight);

    }
}

fn main() {

    let small_dimensions = Dimensions {
        height: 30,
        length: 50,
        width: 40,
    };
    let small_box = ShippingBox::new(small_dimensions, 15.6, Color::Red);


small_box.print();
    // characteristics.new;
}
