// clippy1.rs

fn main() {
    let radius = 5.0f32;
    let area = std::f32::consts::PI * radius * radius;

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}