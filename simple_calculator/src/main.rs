use std::io;


fn main() {
    println!("Programm started!");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operation = String::new();

    println!("Enter number 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter number 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    println!("Enter operation (e.g.: +, -, *, or /):");
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    let num1: f64 = input1.trim().parse().expect("Please type a number!");
    let num2: f64 = input2.trim().parse().expect("Please type a number!");
    let op = operation.trim();

    match op {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Error: Division by zero!");
            }
        },
        _ => println!("Invalid operator! You entered: '{}'", op),
    }
}
