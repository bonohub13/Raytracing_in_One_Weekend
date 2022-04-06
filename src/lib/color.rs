pub fn write_color(pixel_color: crate::Color) {
    let pc = pixel_color * 255.999;
    let x = pc.x() as i64;
    let y = pc.y() as i64;
    let z = pc.z() as i64;

    println!("{} {} {}", x, y, z);
}
