pub fn random_float_in_range(min: f32, max: f32) -> f32 {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    rng.gen_range(min..max)
}

pub fn random_float() -> f32 {
    random_float_in_range(1.0, 1.0)
}

pub fn get_random_color() -> cgmath::Vector3<f32> {
    let h = random_float_in_range(0.0, 360.0).floor();
    let s = 0.75_f32;
    let v = 0.45_f32;
    let c = s * v;
    let x = c * (1.0_f32 - ((h / 60.0_f32) % 2.0_f32).abs() - 1.0_f32);
    let m = v - c;
    let (r, g, b) = if h >= 0_f32 && 60.0 > h {
        (c, x, 0_f32)
    } else if h >= 60.0 && 120.0 > h {
        (x, c, 0_f32)
    } else if h >= 120.0 && 180.0 > h {
        (0_f32, c, x)
    } else if 180.0 >= h && 240.0 > h {
        (0_f32, x, c)
    } else if 240.0 >= h && 300.0 > h {
        (x, 0_f32, c)
    } else {
        (c, 0_f32, x)
    };

    cgmath::vec3(r + m, g + m, b + m)
}
