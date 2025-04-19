fn main() {
    println!("Hello, world!");
    let arr2d: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("{:?}", arr2d);
    // Accessing elements
    println!("Element at (0, 0): {}", arr2d[0][0]);
    println!("Element at (1, 1): {}", arr2d[0][1]);

}
