use gdnative::core_types::Vector2;
pub fn add(a: Vector2, b: Vector2) -> Vector2 {
    let x = a.x + b.x;
    let y = a.y + b.y;
    Vector2 { x: x, y: y }
}

pub fn sub(a: Vector2, b: Vector2) -> Vector2 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    Vector2 { x: x, y: y }
}

pub fn cal_dir(a: Vector2, b: Vector2) -> f32 {
    let s = sub(a, b);

    let mut j = s.y.atan2(s.x) * 180.0 / 3.1415926;
    if j >= -270.0 && j < -90.0 {
        j = 270.0 + 180.0 + j
    } else {
        j = j + 90.0;
    }
    j
}

// Calculate Direction
pub fn cal_d(d: f32) -> u8 {
    match d {
        d if d >= 337.5 && d < 360.0 || d <= 0.0 && d < 22.5 => return 0,
        d if d >= 22.5 && d < 67.5 => return 1,
        d if d >= 67.5 && d < 112.5 => return 2,
        d if d >= 112.5 && d < 157.5 => return 3,
        d if d >= 157.5 && d < 202.5 => return 4,
        d if d >= 202.5 && d < 247.5 => return 5,
        d if d >= 247.5 && d < 292.5 => return 6,
        d if d >= 292.5 && d < 337.5 => return 7,
        _ => return 0,
    };
}
