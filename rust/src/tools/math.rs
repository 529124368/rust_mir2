use gdnative::core_types::Vector2;
pub fn add(a: Vector2, b: Vector2) -> Vector2 {
    let x = a.x + b.x;
    let y = a.y + b.y;
    Vector2 { x, y }
}

pub fn sub(a: Vector2, b: Vector2) -> Vector2 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    Vector2 { x, y }
}

pub fn cal_dir(a: Vector2, b: Vector2) -> f32 {
    let s = sub(a, b);

    let mut j = s.y.atan2(s.x) * 180.0 / std::f32::consts::PI;
    if (-270.0..-90.0).contains(&j) {
        j += 270.0 + 180.0
    } else {
        j += 90.0;
    }
    j
}

// Calculate Direction
pub fn cal_d(d: f32) -> u8 {
    let d = match d {
        d if (337.5..360.0).contains(&d) || (0.0..22.5).contains(&d) => 0,
        d if (22.5..67.5).contains(&d) => 1,
        d if (67.5..112.5).contains(&d) => 2,
        d if (112.5..157.5).contains(&d) => 3,
        d if (157.5..202.5).contains(&d) => 4,
        d if (202.5..247.5).contains(&d) => 5,
        d if (247.5..292.5).contains(&d) => 6,
        d if (292.5..337.5).contains(&d) => 7,
        _ => return 0,
    };
    d as u8
}
