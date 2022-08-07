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
pub fn cal_d(d: f32) -> u32 {
    let d = d as u32;
    if d < 15 || d >= 345 && d < 360 {
        return 6;
    }
    if d >= 15 && d < 35 {
        return 12;
    }
    if d >= 35 && d < 55 {
        return 2;
    }
    if d >= 55 && d < 75 {
        return 13;
    }
    if d >= 75 && d < 105 {
        return 7;
    }
    if d >= 105 && d < 125 {
        return 14;
    }
    if d >= 125 && d < 145 {
        return 3;
    }
    if d >= 145 && d < 165 {
        return 15;
    }
    if d >= 165 && d < 195 {
        return 4;
    }
    if d >= 195 && d < 215 {
        return 8;
    }
    if d >= 215 && d < 235 {
        return 0;
    }

    if d >= 235 && d < 255 {
        return 9;
    }
    if d >= 255 && d < 285 {
        return 5;
    }
    if d >= 285 && d < 305 {
        return 10;
    }
    if d >= 305 && d < 325 {
        return 1;
    }
    if d >= 325 && d < 345 {
        return 11;
    }

    return 0;
}
