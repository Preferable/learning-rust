fn test(n:i32) {
    let mut result = 1;

    if n < 0 { print!("0"); return; }
    for i in 1..n + 1 { result *= i; }
    print!("{}", result);
}