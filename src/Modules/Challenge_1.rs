fn compute_triangle_area(base: i32, height: i32) -> f32 {
    (base as f32) * (height as f32) * 0.5
}

mod shapes {
    pub fn compute_area_of_triangle(base: i32, height: i32) {
        println!("The area of the triangle is: {}", super::compute_triangle_area(base, height));
    }
}