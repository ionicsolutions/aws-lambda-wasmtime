#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::function::lambda::Event;
use crate::bindings::exports::component::function::lambda::Guest;
use crate::bindings::exports::component::function::lambda::Response;

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

struct Component;

impl Guest for Component {
    fn handler(event: Event) -> Response {
        Response {
            factors: calculate_prime_factors(event.number),
        }
    }
}

bindings::export!(Component with_types_in bindings);
