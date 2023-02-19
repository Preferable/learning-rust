#![allow(dead_code)] 
//declare a structure
struct Car {
    owner_age:i32,
}
struct Motorbike {
    owner_age:i32,
}
//declare a trait
trait Drive {
    fn can_drive(&self)->i32;
}
//implement the trait
impl Drive for Car{
    fn can_drive(&self)->i32{
        if self.owner_age >= 18 {
            1
        } else {
            0
        }
        -1
    }
}
impl Drive for Motorbike{
    fn can_drive(&self)->i32{
        if self.owner_age >= 14 {
            1
        } else {
            0
        }
        -1
    }
}