use serde_json::{Value, Result};
use std::collections::{BTreeMap, HashMap};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <JSON event>");
        std::process::exit(1);
    }

    let event: Value = serde_json::from_str(&args[1])
        .expect("Failed to parse the input JSON event");

    let mut number: i64 = event.get("number")
        .and_then(|v| v.as_i64())
        .expect("Missing or invalid \"number\" field");


    let mut prime_factors = BTreeMap::new();

    // Handle factor of 2
    let mut count = 0;
    while number % 2 == 0 {
        number /= 2;
        count += 1;
    }
    if count > 0 {
        prime_factors.insert(2, count);
    }

    // Handle odd factors
    let mut factor = 3;
    while factor * factor <= number {
        count = 0;
        while number % factor == 0 {
            number /= factor;
            count += 1;
        }
        if count > 0 {
            prime_factors.insert(factor, count);
        }
        factor += 2;
    }
    // Handle remaining prime factor greater than sqrt(number)
    if number > 1 {
        prime_factors.insert(number, 1);
    }

    // Expand prime factors into a full list of factors
    let full_factors: Vec<i64> = prime_factors
        .iter()
        .flat_map(|(&prime, &count)| std::iter::repeat(prime).take(count as usize))
        .collect();

    let result = HashMap::from([("factors", full_factors)]);
    let output = serde_json::to_string(&result)?;
    println!("{}", output);

    Ok(())
}
