fn main() {
    let term =7;
    let result = fibonacci(term);
    println!("{}", result);
}

fn fibonacci(term: i32) -> i32 {
    if term == 0 {
        return 0;
    } else if term == 1 {
        return 1;
    } else {
        return fibonacci(term - 1) + fibonacci(term - 2);
    }
}