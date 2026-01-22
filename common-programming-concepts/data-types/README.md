# Data Types
Rust is a statically typed language, which means that it must know the types of all variables at compile time.

## Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: 
- integers, 
- floating-point numbers, 
- Booleans and 
- Characters.

### Integers Types

| Length                  | Signed | Unsigned |
|-------------------------|--------|----------|
| 8-bit                   | i8     | u8       |
| 16-bit                  | i16    | u16      |
| 32-bit                  | i32    | u32      |
| 64-bit                  | i64    | u64      |
| 128-bit                 | i128   | u128     |
| Architecture-dependent  | isize  | usize    |

### Floating-Point Types
Rust’s floating-point types are ``f32`` and ``f64``, which are 32 bits and 64 bits in size, respectively.

```Rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
### Numeric Operations
```Rust
fn main() {
    let sum = 5 + 10;                // addition
    let difference = 95.5 - 4.3;     // subtraction
    let product = 4 * 30;            // multiplication
    let quotient = 56.7 / 32.2;      // division
    let truncated = -5 / 3;          // Results in -1
    let remainder = 43 % 5;          // remainder
}
```

## The Boolean Type
Rust has two possible values: ``true`` and ``false``.
```Rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## The Character Type
Rust’s ``char`` type is the language’s most primitive alphabetic type.
```Rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.

## The Tuple Type
```Rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
```Rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
```Rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

## The Array Type
Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length.
```Rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
Arrays are more useful when you know the number of elements will not need to change.
```Rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

### Array Element Access
You can access elements of an array using indexing, like this:
```Rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
