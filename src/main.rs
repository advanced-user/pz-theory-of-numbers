use std::io;

fn main() {
    solve_equations();
}

fn solve_equations() {
    println!("Enter module m: ");
    let m = read_integer();
    if m < 0 {
        panic!("Modulus must be greater than zero")
    }

    println!("Enter a: ");
    let a = read_integer();

    println!("Enter b: ");
    let b = read_integer();

    let first_remainder = calculate_mod(a, m);
    println!("{} mod {} = {}", a, m, first_remainder);

    let second_remainder = calculate_degree_by_module(a, b, m);
    println!("{}^{} mod {} = {}", a, b, m, second_remainder);

    solve_linear_equation(a, b, m);
}

fn read_integer() -> i128 {
    let mut m_string = String::new();
    io::stdin().read_line(&mut m_string).expect("Something went wrong :(");

    let trimmed = m_string.trim();

    trimmed.parse::<i128>()
        .expect(&*format!("Unable to convert input to integer: {}", trimmed))
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
    let gcd = gcd(a, b);
    if gcd == -1 {
        return;
    }

    println!("gcd = {}", gcd);
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 || b < 0 {
        println!("Unable to compute gcd. Numbers a and b must not be negative");
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