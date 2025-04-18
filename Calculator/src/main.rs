// The add function takes two integers as input and returns their sum.
// The main function initializes two integers, calls the add function, and prints the result.
// The add function is defined to take two i32 parameters and return their sum.
fn main() {
    println!("The SUM program is running!");
    let a = 5;
    let b = 10;
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
    let result = subtract(a, b);
    println!("The difference of {} and {} is {}", a, b, result);
    let result = multiply(a, b);
    println!("The product of {} and {} is {}", a, b, result);
    let result = divide(a, b);
    println!("The quotient of {} and {} is {}", a, b, result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    b - a
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    if b != 0 {
        b / a
    } else {
        panic!("Cannot divide by zero");
    }
}