fn main() {
    // statement
    let value = 10;
    // expression
    let duplicated = duplicate_value(value);
    println!("value: {value} duplicated: {duplicated}");

    // expression with scope block
    let num = {
        // run this scope block expressin, return the result and assign it to `num`
        let x = 3;
        x * 5
    };
    println!("num {num}");
}

fn duplicate_value(val: i32) -> i32 {
    val * 2
}
