struct Point {
    x: i32,
    y: i32,
}

fn test(point1: Point, point2: Point) -> f32 {
    let distance = (point1.x - point2.x).pow(2) + (point1.y - point2.y).pow(2);
    let a = distance as f32;
    a.sqrt()
}