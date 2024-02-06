fn main() {
    // base condition
    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // multiple condition
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // condition in let statement
    let cond = false;
    let number = if cond { 5 } else { 10 };
    // let number = if cond { 5 } else { "hola" }; // error: expected integer, found `&str`
    println!("The value of number is: {number}");

    // loop
    // return value from loop
    let mut count = 0;
    let result = loop {
        count = count + 1;
        if count > 10 {
            break count;
        }
    };
    println!("loop count: {result}");

    // loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut num = 10;

        loop {
            println!("num = {num}");

            if count == 2 {
                println!("break out loop");
                break 'counting_up;
            }

            if num == 8 {
                println!("break inner loop");
                break;
            }

            num -= 1;
        }

        count += 1;
    }

    // conditional loop: while
    let mut count = 0;
    while count < 5 {
        count += 1;
        println!("count = {count}")
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for e in a {
        println!("value = {e}");
    }

    for num in (1..4).rev() {
        println!("num = {num}");
    }
}
