fn main() {
    println!("Hello, world!");

    let y = {
        let x = 1;
        x + 2 // expression doesn't have a semicolon at the end of line
    };

    second_function(5);

    let x_five: i32 = five();

    println!("Function five return {x_five}")
}

fn second_function(x: i32) {
    println!("I'm a second function {x}")
}

fn five() -> i32 {
    10 // the last value in a function is a return value, there is no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1 // adding semicolon will break the code
}