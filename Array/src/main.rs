/// This program demonstrates basic usage of arrays in Rust,
/// including immutable and mutable arrays.
///
/// # Explanation
///
/// - `arr` is an immutable array of 5 integers.
/// - `arr2` is a mutable array, declared with the `mut` keyword,
///   allowing modification of its elements.
///
/// # Use of `mut`
///
/// In Rust, variables are immutable by default.
/// The `mut` keyword allows a variable to be mutable,
/// meaning its value can be changed after initialization.
/// Here, `arr2` is mutable, so we can modify its elements.
/// Without `mut`, attempting to change an element would cause a compile error.
fn main() {
    // Print a greeting message
    println!("Hello, world!");

    // Declare an immutable array of 5 integers
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Print the entire array using debug format
    println!("The value of arr{:?}", arr);
    // Print the first element of the array
    println!("The value of arr[0]: {}", arr[0]);

    // Declare a mutable array of integers
    let mut arr2 = [55, 66, 77, 88, 99];
    // Modify the third element of the mutable array
    arr2[2] = 100;
    // Print the modified element
    println!("The value of arr2[2]: {}", arr2[2]);
}
