fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let num: i8 = 127;
    println!("{num}");

    let num2: i16 = 32767;
    println!("{num2}");

    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // Results in -1

    println!("{quotient}");
    println!("{truncated}");

    let arrayNums: [i8; 5] = [1, 2, 3, 4, 5];
    let num0 = arrayNums[0];
    println!("{num0}");
}
