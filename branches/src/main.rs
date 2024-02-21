fn main() {
    let x = 7;

    if x < 5 {
        println!("Statement is true");
    } else {
        println!("Statement is false");
    }

    if x != 0 {
        println!("Number isn't a zero");
    }

    let condition = true;
    let x = if condition { 5 } else { 10 };

    println!("The x is: {x}");

    // if x { <- Rust dont evaluate another types to boolean
    //     println!("is it true?!");
    // }

    loop {
        println!("again!");
        break;
    } // stop by pressing CTRL+C shown as ^C

    let mut counter = 0;
    let loop_result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {loop_result}");

    // multiple loops - loop label
    let mut multi_loops_count = 0;
    'counting_up: loop { // <- loop label only has opening single qoute sign
        println!("count = {multi_loops_count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if multi_loops_count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        multi_loops_count += 1;
    }
    println!("End count = {multi_loops_count}");

    // while
    let mut while_number =3;

    while while_number != 0 { // reduces if/breaks statements in code
        println!("{while_number}");

        while_number -= 1;
    }
    print!("LIFTOFF!!!");

    // looping in collection
    let a = [10, 20, 30, 40, 50];
    let mut array_index = 0;

    while array_index < 5 { // could perform panic while index is out of bounds
        println!("the value is: {}", a[array_index]);

        array_index += 1;
    }

    // for in loop
    for element in a {
        println!("The value is: {element}")
    }

    // with generated range in reverse
    for number in (1..4).rev() {
        println!("The number is {number}")
    }

}
