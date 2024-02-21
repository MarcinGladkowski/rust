use std::io;

fn main() {
    println!("Hello, world!");

    let mut temperature = String::new();
    let mut metric_scale = String::new();
    println!("Please provide temperature value: ");
    let stdin = io::stdin(); // got stdin
    stdin.read_line(&mut temperature);

    // validate inouit

    println!("Please provide type: Fahrenheit(F) or Celsius (C)");
    stdin.read_line(&mut metric_scale);

    // validate input

    println!("You provided {temperature} and metric {metric_scale}")
}
