struct Color(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);

    println!("Hue {}", black.0);
    println!("Value {}", black.1);
    println!("Chroma {}", black.2);
}
