use super::{utils::*, Vec3};
#[test]
fn test_dot() {
    let v = Vec3::new(2.0, 3.0, 5.0);
    let u = Vec3::new(7.0, 11.0, 13.0);
    let target = 112.0; // 14 + 33 + 65

    assert_eq!(target, dot(&v, &u));
}

#[test]
fn test_cross() {
    let v = Vec3::new(2.0, 3.0, 5.0);
    let u = Vec3::new(7.0, 11.0, 13.0);
    let target = Vec3::new(
        -16.0, // 39 - 55
        9.0,   // 35 - 26
        1.0,   // 22 - 21
    );

    assert_eq!(target, cross(&v, &u));
}

#[test]
fn test_unit_vector() {
    let v = Vec3::new(2.0, 3.0, 5.0);
    let target = Vec3::new(
        2.0 / 38.0.sqrt(), // 2.0 / (4.0 + 9.0 + 25.0).sqrt()
        3.0 / 38.0.sqrt(), // 3.0 / (4.0 + 9.0 + 25.0).sqrt()
        5.0 / 38.0.sqrt(), // 5.0 / (4.0 + 9.0 + 25.0).sqrt()
    );

    assert_eq!(target, unit_vector(&v));
}

#[test]
fn test_reflect() {
    let v = Vec3::new(2.0, 3.0, 5.0);
    let u = Vec3::new(7.0, 11.0, 13.0);
    let result = reflect(&v, &u);
    let target = Vec3::new(
        -1566.0, // 2.0 - 2.0 * (112.0) * 7.0
        -2461.0, // 3.0 - 2.0 * (112.0) * 11.0
        -2907.0, // 5.0 - 2.0 * (112.0) * 13.0
    );

    assert_eq!(target, result);
}

#[test]
fn test_refract() {
    let uv = Vec3::new(2.0, -3.0, 5.0);
    let n = Vec3::new(-7.0, 11.0, -13.0);
    let etai_over_etat = 17.0;
    let result = refract(&uv, &n, etai_over_etat);
    let scala = -44216.0.sqrt();
    // cos_theta: 1.0
    // r_out_perp: [17.0 * 5.0, 17.0 * -8.0, 17.0, -8.0]
    // r_out_prallel: [scala * -7.0, scala * 11.0, scala * -13.0]
    let target = Vec3::new(
        -85.0 + scala * -7.0,   // -85.0 + scala * -7.0
        136.0 + scala * 11.0,   // 136.0 + scala * 11.0
        -136.0 + scala * -13.0, // -136.0 + scala * -13.0
    );

    assert_eq!(target, result);
}
