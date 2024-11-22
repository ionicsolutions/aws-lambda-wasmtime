use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

fn calculate_prime_factors(mut number: i64) -> Vec<i64> {
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

    prime_factors
}

#[derive(Deserialize, Serialize)]
struct Event {
    number: i64,
}

#[derive(Serialize)]
struct Factors {
    factors: Vec<i64>,
}

fn receive_event<T: DeserializeOwned>() -> T {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();
    serde_json::from_slice(&buf).unwrap()
}

fn send_response<T: Serialize>(value: &T) {
    io::stdout()
        .write_all(serde_json::to_string(&value).unwrap().as_bytes())
        .unwrap();
    io::stdout().write_all("\n".as_bytes()).unwrap();
}

#[no_mangle]
pub fn lambda_handler() {
    let event = receive_event::<Event>();

    let factors = Factors {
        factors: calculate_prime_factors(event.number),
    };

    send_response(&factors);
}
