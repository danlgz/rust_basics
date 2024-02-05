fn main() {
    // the last defintion is that the compiler "see".
    let x = 5;

    let x = x + 1;

    {
        // new scope: changes only
        let x = x * 2;
        println!("x value in the inner scope: {x}");
    }

    println!("x value: {x}");

    // shadowing vs mut
    // shadowng allow "redefine" a variable
    // mut just can reasign the value but with the same type (this is not "redefine")
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // raise error: expected `&str`, found `usize`
    // let mut new_spaces = "  ";
    // new_spaces = new_spaces.len();
}
