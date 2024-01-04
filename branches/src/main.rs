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

    // if x { <- Rust dont evaluate another types to boolean
    //     println!("is it true?!");
    // }
}
