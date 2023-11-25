fn main() {

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // shadowing
    let y = 10;
    let y = 20;

    {
        let y = 30;
        println!("The value of y variable in scope is: {y}");
    }
    println!("The value of y variable is: {y}");

    // by shadowing we can change the type of variable
    let spaces = "   ";
    let spaces = spaces.len();

    let name_length: u32 = "marcin".parse().expect("Not a number");

    let min: i8 = -127;
}
