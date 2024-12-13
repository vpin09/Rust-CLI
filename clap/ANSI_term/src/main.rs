use ansi_term::Color;

fn main() {
  println!("this is {} in color,,{} in color,{} in color",
Color::Red.paint("Red"),Color::Blue.paint("Blue"),Color::Green.paint("Green"));
}
