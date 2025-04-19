/// This program demonstrates basic usage of arrays in Rust,
/// including array declaration, element access, and arithmetic operations.
///
/// # Explanation
///
/// - `arr` and `arr2` are immutable arrays of integers.
/// - The variable `x` is assigned the sum of the 4th element of `arr`
///   and the 3rd element of `arr2`.
/// - The program prints the value of `x`.
///
/// # Details
///
/// - Arrays in Rust are fixed-size collections of elements of the same type.
/// - Elements are accessed using zero-based indexing.
/// - Here, `arr[3]` accesses the 4th element (value 4),
///   and `arr2[2]` accesses the 3rd element (value 30).
/// - The sum of these elements (4 + 30 = 34) is stored in `x`.
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [10, 20, 30, 40, 50];
    let x: i32 = arr[3] + arr2[2];
    println!("The value of x is: {}", x);

}
