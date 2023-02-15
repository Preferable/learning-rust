fn test(mut x:i32) {
    let mut count = 1;

    while x > 0 {
        count += 1;
        x -= 3;

        if x == 0 {
            println!("{}", count);
        }
    }
}