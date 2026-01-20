# Guessing Game

This is a simple guide to the Rust Guessing Game, based on the [official Rust Book](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html).

---

## Printing Output

To print a line:
```rust
println!("Guess the number!");
```

## Variables

Create a new mutable string:
```rust
let mut guess = String::new();
```

Declare immutable and mutable variables:
```rust
let apples = 5;      // immutable
let mut bananas = 5; // mutable
```

## Getting Player Input

To get user input:
```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- `io::stdin()` returns an instance of `std::io::Stdin`, which represents the standard input handle.
- `.read_line(&mut guess)` reads a line from standard input and stores it in the mutable string `guess`. The `&` indicates a reference.
- `.read_line` returns a `Result` enum, with variants `Ok` and `Err`.
- `.expect("Failed to read line")` will crash the program and display the message if an error occurs.

If you don’t call `expect`, the program will compile, but you’ll get a warning for not handling the `Result`.

## String Interpolation

Curly brackets `{}` are placeholders in format strings. You can use variable names inside them:
```rust
let x = 5;
let y = 10;
println!("x = {x} and y + 2 = {}", y + 2);
```

## Adding Dependencies

To add a dependency like `rand` for random numbers, edit your `Cargo.toml`:
```toml
[dependencies]
rand = "0.8.5"
```
Then build your project:
```bash
cargo build
```

## Generating a Random Number

To generate a random number between 1 and 100:
```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

## Documentation

To build and open documentation for your dependencies:
```bash
cargo doc --open
```

## Matching Values

Use `match` to compare values:
```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```
- `Ordering` is an enum with variants: `Less`, `Greater`, and `Equal`.
- The `cmp` method compares two values.
- The `match` expression executes code based on the result.

## Converting Input

Convert a string to a number:
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
- `.trim()` removes whitespace from the start and end of the string.
- `.parse()` converts the string to another type (here, `u32`).
- The type annotation `: u32` specifies the desired type.
- If you want to crash on error and show a message, use:
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

## Loop Keyword

Create an infinite loop:
```rust
loop {
    // code
}
```
Use `break` to exit the loop when needed.

---

## Reference

This README is based on the official Rust Book: [Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)