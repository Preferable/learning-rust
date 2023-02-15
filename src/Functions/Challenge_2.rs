fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr = arr_square(arr);
    println!("{:?}", arr);
}

fn arr_square(arr: [i32; 5]) -> [i32;5] {
    let mut arr2 = [0; 5];
    for i in 0..5 {
        arr2[i] = arr[i] * arr[i];
    }
    arr2
}