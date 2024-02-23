use std::io;
use std::io::Stdin;
use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");

    let mut temperature = String::new();
    let mut metric_scale = String::new();
    println!("Please provide temperature value: ");
    let stdin = io::stdin(); // got stdin
    stdin.read_line(&mut temperature);

    // validate input
    println!("Please provide type: Fahrenheit(F) or Celsius (C)");
    stdin.read_line(&mut metric_scale);

    let scales: [&str; 2] = ["F", "C"];

    let is_correct_metric: bool = scales.contains(metric_scale);

    if !is_correct_metric {
        println!("Incorrect metric provided. Only {:?} available", scales);
        return;
    }

    // validate input
    println!("You provided {temperature} and metric {metric_scale}");

    let temperature_value: i32 = temperature.trim().parse().expect("Cannot par10se provided temperature");

}
