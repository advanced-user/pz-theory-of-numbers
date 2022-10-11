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

    solve_first_equation(a, m);
    solve_second_equation(a, b, m);
    solve_third_equation(a, b, m);
}

fn read_integer() -> i32 {
    let mut m_string = String::new();
    io::stdin().read_line(&mut m_string).expect("Something went wrong :(");

    let trimmed = m_string.trim();

    trimmed.parse::<i32>()
        .expect(&*format!("Unable to convert input to integer: {}", trimmed))
}

// a mod m = x
fn solve_first_equation(a: i32, m: i32) {
    let fraction = a / m;

    let mut remainder = a - m * fraction;
    if remainder < 0 {
        remainder += m;
    }

    println!("{} mod {} = {}", a, m, remainder);
}

// a^b mod m = x
fn solve_second_equation(a: i32, b: i32, m: i32) {

}

// a*x ≡ b mod m
fn solve_third_equation(a: i32, b: i32, m: i32) {

}




