use std::io;

fn main() {
    let data_for_equation = read_data_for_equations();
    solve_equations(data_for_equation.0, data_for_equation.1, data_for_equation.2);

    let range = read_range();
    generate_prime_number(range.0, range.1);
}

fn read_data_for_equations() -> (i128, i128, i128) {
    println!("Enter module m: ");
    let m = read_integer();
    if m < 0 {
        panic!("Modulus must be greater than zero")
    }

    println!("Enter a: ");
    let a = read_integer();

    println!("Enter b: ");
    let b = read_integer();

    (a, b, m)
}

fn read_integer() -> i128 {
    let mut m_string = String::new();
    io::stdin().read_line(&mut m_string).expect("Something went wrong :(");

    let trimmed = m_string.trim();

    trimmed.parse::<i128>()
        .expect(&*format!("Unable to convert input to integer: {}", trimmed))
}

fn read_range() -> (i128, i128) {
    println!("\nGenerating a prime number in a given range");

    println!("Enter the beginning of the range: ");
    let a = read_integer();

    println!("Enter the end of the range: ");
    let b = read_integer();

    (a, b)
}

fn solve_equations(a: i128, b: i128, m: i128) {
    let first_remainder = calculate_mod(a, m);
    println!("\n{} mod {} = {} \n", a, m, first_remainder);

    let second_remainder = calculate_degree_by_module(a, b, m);
    println!("{}^{} mod {} = {} \n", a, b, m, second_remainder);

    println!("{}*x ≡ {} mod {}", a, b, m);
    solve_linear_equation(a, b, m);
}

// a mod m = x
fn calculate_mod(a: i128, m: i128) -> i128 {
    let fraction = a / m;

    let mut remainder = a - m * fraction;
    if remainder < 0 {
        remainder += m;
    }

    remainder
}

// a^b mod m = x
fn calculate_degree_by_module(mut a: i128, mut b: i128, m: i128) -> i128 {
    let mut remainder = 1;

    while b != 0 {
        if calculate_mod(b, 2) == 1 {
            remainder = calculate_mod(remainder * a, m);
        }

        b /= 2;
        a = calculate_mod(a * a, m);
    }

    remainder
}

// a*x ≡ b mod m
fn solve_linear_equation(a: i128, b: i128, m: i128) {
    if !is_relatively_prime_numbers(a, m) {
        println!("It is impossible to solve this equation: {}*x ≡ {} mod {}", a, b, m);
        return;
    }

    let phi = phi(m);
    println!("φ({}) = {}", m, phi);

    let a_ = calculate_degree_by_module(a, phi - 1, m);
    let result = calculate_mod(b * a_ ,m);

    println!("x = {}", result);
}

fn is_relatively_prime_numbers(a: i128, m: i128) -> bool {
    let gcd = gcd(a, m);
    println!("gcd({}, {}) = {}", a, m, gcd);

    if gcd != 1 {
        return false;
    }

    true
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 || b < 0 {
        println!("Unable to calculate GCD. Numbers a and b must not be negative.");
        return -1;
    }

    if a == 0 {
        return b;
    }

    while b != 0 {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

fn phi(m: i128) -> i128 {
    let mut result = 0;
    for i in 1..m {
        if gcd(i, m) == 1 {
            result += 1;
        }
    }

    result
}

fn generate_prime_number(a: i128, b: i128) {

}