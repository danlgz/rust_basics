fn main() {
    // a constant value must be an constant expression, cannot be assigned based on a value calculated in execution runtime
    const THREE_HOUR_IN_SECONS: u32 = 60 * 60 * 3;
    println!("3 hour in seconds: {THREE_HOUR_IN_SECONS}");

    let x = 3;
    println!("Value of x: {x}");

    // Error: cannot assign twice to immutable variable
    // x = 4;
    // println!("Value of x: {x}");

    // Now is mutable!
    let mut y = 4;
    println!("Value of y: {y}");

    y = 5;
    println!("Value of y: {y}");
}
