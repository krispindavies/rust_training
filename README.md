## Core Notes:

Watching: https://www.youtube.com/watch?v=rQ_J9WH6CGk
Currently 1:06:00 in.

Comments are C++ style:
`//` for a comment line
`/* ... */` for block comments
`//!` for documentation

Variables are declared with the `let` keyword
`let x: i32 = 10;`
If you want it to be mutable, use the `mut` keyword.
`let mut x: i32 = 10;`

Primitive data types are:
- signed int types: `i8`, `i16`, `i32`, `i64`, `i128`
- unsigned int types: `u8`, `u16`, `u32`, `u64`, `u128`
- float types: `f32`, `f64`
- boolean types: `bool` as `true` or `false`
- character (unicode): `char` as `a`

Main compound types are:
- array types: `[type; size]` as `let numbers: [i32; 5] = [1, 2, 3, 4, 5];`
- tuple types: `(type1, type2, ..., typeN)` as `let employee: (String, i32, bool) = ("Alice", 30, false);`
- slice types: `&type` as `let number_slice: &[i32] = &[1, 2, 3, 4, 5];`
                       or `let book_slice: &str = "The Way of Kings";`
  *Remember that slices are always borrowed!*
- string types: `String` as `let my_string: String = "Halleluja!".to_string();`
                         or `let my_string: String = String::from("Halleluja!");`

On slices:
- You can think of a slice as a reference to a variable or a subsection of a variable.
- You can create a slice from another variable as:
  `let my_string: String = "Halleluja!".to_string();`
  `let slice_a: &str = &my_string;`
  `let slice_b: &str = &my_string[0..5];`

On functions:
- They start with the `fn` keyword:
  `fn main() {`
  `}`
- The `main` function is always the entry point of a program.
- Functions can be defined in any order in the file, rust compiler will just figure it out.
- Arguments are in the `name: type` format and comma separated:
  `fn my_function(length: f32, width: f32) {`
- Return type is specified as ` -> type` as:
  `fn add(a: f32, b: f32) -> f32 {`
  `  return a + b;`
  `}`
  -or-
  `fn add(a: f32, b: f32) -> f32 {`
  `  a + b` <-- Will return the result of the function's last line because we didn't put the semicolon.
  `}`
- You can define a lambda function as:
  `fn my_function(length: f32, width: f32) {`
  `  let my_lambda = {`
  `    let inner_x: i32 = 10;`
  `    let inner_y: i32 = 9;`
  `    inner_x * inner_y`
  `  };`
  `  println!("Length {}, width {}, inner {}", length, width, my_lambda);`
  `}`

On naming conventions:
- Function names are `snake_case`
- Variable names are `snake_case`
- Types are `CamelCase`
- Global constants are `UPPER_SNAKE_CASE`

## Useful Macros:

- `println!("My text {}", value)` will print a line to the console with a trailing newline.
  - You can use debug printing with `{:?}` as the entry.
- `format!("My text {}", value)` to format a string.

