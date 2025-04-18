/// @file main.rs
/// @brief This program demonstrates basic arithmetic operations: addition, subtraction, multiplication, and division.
///
/// The main function initializes two integers and calls the respective functions to perform these operations,
/// printing the results to the console.
fn main() {
    println!("The SUM program is running!");
    let a = 5; ///< First operand
    let b = 10; ///< Second operand

    /**
     * @brief The 'let' keyword is used to declare a new variable named 'result'.
     * This variable stores the output of the function call.
     *
     * Here, 'add(a, b)' calls the 'add' function with 'a' and 'b' as arguments.
     * The function calculates the sum of 'a' and 'b' and returns the result.
     * This returned value is then stored in the 'result' variable.
     *
     * Using 'let' allows us to create a variable that holds the function's output,
     * so we can use it later, such as printing it to the console.
     */
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    // Perform subtraction and print the result
    let result = subtract(a, b);
    println!("The difference of {} and {} is {}", a, b, result);

    // Perform multiplication and print the result
    let result = multiply(a, b);
    println!("The product of {} and {} is {}", a, b, result);

    // Perform division and print the result
    let result = divide(a, b);
    println!("The quotient of {} and {} is {}", a, b, result);
}

/// @brief The 'add' function takes two parameters 'a' and 'b' of type i32 (32-bit signed integers).
/// @param a First integer operand
/// @param b Second integer operand
/// @return The sum of the two input integers
///
/// The '-> i32' syntax indicates that this function returns a value of type i32.
/// The function body consists of a single expression 'a + b' which is implicitly returned.
/// This means the function calculates the sum of 'a' and 'b' and returns it to the caller.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns the difference of two integers (b - a)
fn subtract(a: i32, b: i32) -> i32 {
    b - a
}

/// Returns the product of two integers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Returns the quotient of two integers (b / a), panics if dividing by zero
fn divide(a: i32, b: i32) -> i32 {
    if b != 0 {
        b / a
    } else {
        panic!("Cannot divide by zero");
    }
}
