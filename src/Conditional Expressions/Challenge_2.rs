fn test(a: i32, operator: char, b: i32) {
    if a == 0 || b == 0 {
        println!("Division by 0 is undefined");
    } else {
        match operator {
            '+' => println!("{}", a + b),
            '-' => println!("{}", a - b),
            '*' => println!("{}", a * b),
            '/' => println!("{}", a / b),
            '%' => println!("{}", a % b),
            _ => println!("invalid operator"),
        }
    }
}