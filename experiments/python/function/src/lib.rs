use serde::Serialize;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn flip(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

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

#[link(wasm_import_module = "host")]
extern "C" {
    fn report_factor(factor: i64);
}

#[no_mangle]
pub fn calculate_and_report(number: i64) {
    let factors = calculate_prime_factors(number);
    for factor in factors {
        unsafe {
            report_factor(factor);
        }
    }
}

#[no_mangle]
pub fn read_from_env_and_report() {
    let number = std::env::var("NUMBER")
        .expect("NUMBER must be set")
        .parse::<i64>()
        .expect("NUMBER must be an integer");
    calculate_and_report(number);
}

#[no_mangle]
pub fn read_from_args_and_report() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <number>");
        std::process::exit(1);
    }
    let number = args[1].parse::<i64>().expect("NUMBER must be an integer");
    calculate_and_report(number);
}

#[derive(Serialize)]
struct Factors {
    factors: Vec<i64>,
}

#[no_mangle]
pub fn calculate_and_print_json(number: i64) {
    let prime_factors = Factors {
        factors: calculate_prime_factors(number),
    };
    let json = serde_json::to_string(&prime_factors).unwrap();
    println!("{}", json);
}

#[no_mangle]
pub fn calculate_and_store_json(number: i64) -> *mut u8 {
    let prime_factors = Factors {
        factors: calculate_prime_factors(number),
    };
    let mut factors = serde_json::to_string(&prime_factors)
        .unwrap()
        .as_bytes()
        .to_owned();
    let factors_ptr = factors.as_mut_ptr();
    std::mem::forget(factors);
    factors_ptr
}

#[no_mangle]
pub fn calculate_and_store_json_no_forget(number: i64) -> *mut u8 {
    let prime_factors = Factors {
        factors: calculate_prime_factors(number),
    };
    let mut factors = serde_json::to_string(&prime_factors)
        .unwrap()
        .as_bytes()
        .to_owned();
    factors.as_mut_ptr()
}
