fn test_divisibility_by_3_4(a:i32) -> i32{
    if a % 3 == 0 && a % 4 == 0 {
        return 0;
    }
    if a % 4 == 0 {
        return 2;
    }
    if a % 3 == 0 {
        return 1;
    }
    return -1
}

fn main() {
    let mut num;
    println!("Output: {}", test_divisibility_by_3_4(num));
}