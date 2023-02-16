fn test(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
    my_vec.pop();
    println!("After pop: {:?}", my_vec);
    let len = my_vec.len();
    let middle = len / 2;
    my_vec.remove(middle);
    println!("After remove: {:?}", my_vec);
    let sum = my_vec.iter().sum();
    my_vec.push(sum);
    println!("After push: {:?}", my_vec);
    my_vec
 }