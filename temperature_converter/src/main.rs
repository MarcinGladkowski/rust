use std::io;

fn main() {
    println!("Hello, world!");

    let mut temperature = String::new();
    println!("Please provide temperature value: ");
    let stdin = io::stdin(); // got stdin
    stdin.read_line(&mut temperature).expect("Error while provide temperature");

    // trim_end() removes '\n' from input
    let unit: char = temperature.trim_end().chars().last().unwrap();

    let value = &temperature[0..temperature.len()-2];

    let scales: [&char; 2] = [&'F', &'C'];

    let is_correct_metric: bool = scales.contains(&&unit);

    if !is_correct_metric {
        println!("Incorrect metric provided. Only {:?} available", scales);
        return;
    }

    // validate input
    println!("You provided {value}");

    let temperature_value: f64 = value.parse().expect("Cannot parse provided temperature");

    if unit.eq(&'F') {
        let result: f64 = temperature_value - 32.0 / 1.8;
        print!("{}", result);
    }

    if unit.eq(&'C') {
        // calculate from C + F
        let result: f64 = temperature_value * 1.8 + 32.0;

        print!("{}", result);
    }
}
