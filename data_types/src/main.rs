fn main() {
    // `.parse()` needs the type annotation (`:u32`);
    let x: u32 = "42".parse().expect("invalid number value");
    println!("value: {x}");

    println!("== PRIMITIVE TYPES ==");

    // decimal
    let dec = 10_800;
    println!("decimal value: {dec}");

    // hexadecimal
    let hex = 0xff; // 255 decimal
    println!("hexadecimal value: {hex}");

    // ocatagonal
    let oct = 0o77; // 63 octogonal
    println!("octagonal value: {oct}");

    // binary
    let bin = 0b1111_0000; // 240 binary
    println!("binary value: {bin}");

    let by = b'A'; // 65 byte
    println!("byte value: {by}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("chars: {c}, {z}, {heart_eyed_cat}");

    // float
    let x = 2.34; // f64 by default: in modern CPUs f64 provides more precision
    let y: f32 = 3.45; // f32
    println!("x value {x}");
    println!("y value {y}");

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("booleans: {t}, {f}");

    println!("-- Numeric operations --");
    let x = -5;
    let y = 3;

    let sum = x + y;
    println!("addition: {x} + {y} = {sum}");

    let difference = x - y;
    println!("substraction: {x} - {y} = {difference}");

    let product = x * y;
    println!("product: {x} * {y} = {product}");

    let truncated = x / y; // if are defined as integer the division operation is a truncate
    println!("division (integer: truncated): {x} / {y} = {truncated}");
    {
        let x = -5f64;
        let y = 3f64;
        let quotient = x / y;
        println!("division (float: quotient): {x} / {y} = {quotient}");
    }

    let remainder = x % y;
    println!("remainder: {x} % {y} = {remainder}");

    println!("== COMPOUND TYPES ==");

    // let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1); // specify type
    let (x, y, z) = tup; // tuple destructure
    println!("tuple values: {x}, {y}, {z}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let first = months[0];
    println!("array first value: {first}");

    // define type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("array first value: {first}");

    // initialize array with specific values
    let a = [3; 5];
    let first = a[0];
    println!("array first value: {first}");
}
