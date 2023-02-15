fn test(n:i32) {
    let char = '&';
    let mut count = 1;

    for _i in 0..n {
        for _i in 0..count {
            print!("{}", char);
        }
        print!("\n");
        count = count + 1;
    }
}