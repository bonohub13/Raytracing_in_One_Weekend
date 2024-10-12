use super::{utils::*, Vec3};
#[test]
fn test_dot() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let target = 112_f64; // 14 + 33 + 65

    assert_eq!(target, dot(&v, &u));
}

#[test]
fn test_cross() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let target = Vec3::new(
        -16_f64, // 39 - 55
        9_f64,   // 35 - 26
        1_f64,   // 22 - 21
    );

    assert_eq!(target, cross(&v, &u));
}

#[test]
fn test_unit_vector() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let target = Vec3::new(
        2_f64 / 38_f64.sqrt(), // 2_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
        3_f64 / 38_f64.sqrt(), // 3_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
        5_f64 / 38_f64.sqrt(), // 5_f64 / (4_f64 + 9_f64 + 25_f64).sqrt()
    );

    assert_eq!(target, unit_vector(&v));
}

#[test]
fn test_reflect() {
    let v = Vec3::new(2_f64, 3_f64, 5_f64);
    let u = Vec3::new(7_f64, 11_f64, 13_f64);
    let result = reflect(&v, &u);
    let target = Vec3::new(
        -1566_f64, // 2_f64 - 2_f64 * (112_f64) * 7_f64
        -2461_f64, // 3_f64 - 2_f64 * (112_f64) * 11_f64
        -2907_f64, // 5_f64 - 2_f64 * (112_f64) * 13_f64
    );

    assert_eq!(target, result);
}

#[test]
fn test_refract() {
    let uv = Vec3::new(2_f64, -3_f64, 5_f64);
    let n = Vec3::new(-7_f64, 11_f64, -13_f64);
    let etai_over_etat = 17_f64;
    let result = refract(&uv, &n, etai_over_etat);
    let scala = -44216_f64.sqrt();
    // cos_theta: 1_f64
    // r_out_perp: [17_f64 * 5_f64, 17_f64 * -8_f64, 17_f64, -8_f64]
    // r_out_prallel: [scala * -7_f64, scala * 11_f64, scala * -13_f64]
    let target = Vec3::new(
        -85_f64 + scala * -7_f64,   // -85_f64 + scala * -7_f64
        136_f64 + scala * 11_f64,   // 136_f64 + scala * 11_f64
        -136_f64 + scala * -13_f64, // -136_f64 + scala * -13_f64
    );

    assert_eq!(target, result);
}
