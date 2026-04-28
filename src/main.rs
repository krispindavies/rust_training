fn my_function(length: f32, width: f32) {
  let my_inner_function = {
    let inner_x: i32 = 10;
    let inner_y: i32 = 9;
    inner_x * inner_y
  };
  println!("Length {}, width {}, inner {}", length, width, my_inner_function);
}

fn add(a: f32, b: f32) -> f32 {
  a + b
}

fn main() {
    let my_string: String = "Halleluja!".to_string();
    let slice_a: &str = &my_string;
    let slice_b: &str = &my_string[0..5];
    println!("Hello, world! {} {}", slice_a, slice_b);
    my_function(5.0, 7.0);
    println!("My value {}", add(9.0, 7.3));
}
