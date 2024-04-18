use std::io;

fn main() {
    println!("Hello, world!");

    let mut temperature = String::new();
    let mut metric_scale = String::new();
    println!("Please provide temperature value: ");
    let stdin = io::stdin(); // got stdin
    stdin.read_line(&mut temperature).expect("Error while provide temperature");

    let parsed_metric = metric_scale.trim();

    // validate input
    println!("Please provide type: Fahrenheit(F) or Celsius (C)");
    stdin.read_line(&mut metric_scale).expect("Error while provide temperature metric");

    metric_scale.contains(parsed_metric);

    let scales: [&str; 2] = ["F", "C"];

    let is_correct_metric: bool = scales.contains(&metric_scale.as_str().trim());

    if !is_correct_metric {
        println!("Incorrect metric provided. Only {:?} available", scales);
        return;
    }

    // validate input
    println!("You provided {temperature} and metric {metric_scale}");

    let temperature_value: f64 = temperature.trim().parse().expect("Cannot parse provided temperature");

    if parsed_metric.eq("F") {
        // calculate from F to C
        let result: f64 = temperature_value - 32.0 / 1.8;

        print!("{}", result);
    }

    if parsed_metric.eq("C") {
        // calculate from C + F
        let result: f64 = temperature_value * 1.8 + 32.0;

        print!("{}", result);
    }
}
