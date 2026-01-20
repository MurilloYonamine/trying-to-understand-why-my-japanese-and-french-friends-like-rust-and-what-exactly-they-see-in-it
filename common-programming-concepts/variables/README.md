# Variables and Mutability

When a variable is immutable, once a value is bound to a name, you can’t change that value.
```Rust
let x = 5;
x = 6; // error
```

You can make them mutable by adding mut in front of the variable name.
```Rust
let mut x = 5;
x = 6; // no error
```

Constantes are **ALWAYS** immutable by default. You **CAN'T** use the `mut` keyword here and the type of the value **MUST** be annotated. 

You use `const` instead of `let`.
```Rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
A const variable cannot depend on user input, dynamic allocation and common functions. It doesn't exist a "const" in runtime.

You can't do something like this:
```Rust
const V: Vec<i32> = Vec::new(); // ERROR
```

| Aspect                             | `const`                       | `let` immutable   |
| ----------------------------------- | ----------------------------- | ----------------- |
| Evaluation                         | Compile-time                  | Runtime           |
| Memory                             | No fixed address              | Has address       |
| Can be global                      | Yes                           | No                |
| Type required                      | Yes                           | Inferred          |
| Can call functions                 | Only `const fn`               | Any function      |
| Can contain heap (`Vec`, `String`) | No                            | Yes               |
| Typical use                        | Configs, limits, flags        | Normal variables  |

## Shadowing
```Rust
let y = 5;

let y = y + 1;
{
    let y = y * 2;
    println!("The value of y in the inner scope is: {y}"); // The value of x in the inner scope is: 12
}
println!("The value of y is: {y}"); // The value of x is: 6
```

We can change the type of the value but reuse the same name. The first spaces variable is a string type, and the second spaces variable is a number type. Instead of having two variables, we can reuse the simpler spaces name.
```Rust
let spaces = "   "; // string type
println!("{spaces}");

let spaces = spaces.len(); // number type
println!("{spaces}");
```
> Obs: Don't use mut here!

## Reference

This README is based on the official Rust Book: [Variables and Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)