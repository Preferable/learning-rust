enum Days{
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}
impl Days{
    fn is_weekend(&self)->i32{
      match self{
        &Days::Saturday=>return 1,
        &Days::Sunday=>return 1,
        _=>return 0
      }
    }
}

fn main() {
    let mut check_day = Days::Saturday;
    println!("Is Saturday a weekend ? : {}", check_day.is_weekend());
    check_day = Days::Monday;
    println!("Is Monday a weekend ? : {}", check_day.is_weekend())
}