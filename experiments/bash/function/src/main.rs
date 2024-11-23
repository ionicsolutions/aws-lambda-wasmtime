use serde_json::Value;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <JSON event>");
        std::process::exit(1);
    }

    let event: Value =
        serde_json::from_str(&args[1]).expect("Failed to parse the input JSON event");

    let mut number: i64 = event
        .get("number")
        .and_then(|v| v.as_i64())
        .expect("Missing or invalid \"number\" field");

    let mut prime_factors = Vec::<i64>::new();

    // Handle factor of 2
    while number % 2 == 0 {
        number /= 2;
        prime_factors.push(2);
    }

    // Handle odd factors
    let mut factor = 3;
    while factor * factor <= number {
        while number % factor == 0 {
            number /= factor;
            prime_factors.push(factor);
        }
        factor += 2;
    }

    // Handle remaining prime factor greater than sqrt(number)
    if number > 1 {
        prime_factors.push(number);
    }

    let result = HashMap::from([("factors", prime_factors)]);
    let output = serde_json::to_string(&result).unwrap();
    println!("{}", output);
}
