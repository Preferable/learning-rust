fn test(my_str:String)-> String {
    let mut result = String::new();
    for word in my_str.split_whitespace() {
        if word.starts_with('c') {
            result.push_str(word);
            result.push_str(" ");
        }
    }
    result.pop();
    result
}