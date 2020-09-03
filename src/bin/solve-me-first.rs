/**
 * Read two numbers from the standard io then add it and print out the result
 */
use std::io;

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();

    println!("Please enter number 1st:");
    io::stdin().read_line(&mut str1).ok().expect("read error");

    println!("Please enter number 2st:");
    io::stdin().read_line(&mut str2).ok().expect("read error");

    let num1: i32 = str1.trim().parse().ok().expect("parse error");
    let num2: i32 = str2.trim().parse().ok().expect("parse error");

    println!("Input number 1: {}", num1);
    println!("Input number 2: {}", num2);

    println!("{0} + {1} = {2}", num1, num2, num1 + num2);
}
