use rtiow::{interval::Interval, utils as rt_utils};

const SQRT_N: i32 = 1_000;

fn main() {
    let range = Interval::new(-1.0, 1.0);
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;

    for i in 0..SQRT_N {
        for j in 0..SQRT_N {
            let x = rt_utils::random_in_range(&range);
            let y = rt_utils::random_in_range(&range);

            if (x.powi(2) + y.powi(2)) < 1.0 {
                inside_circle += 1;
            }

            let x = 2.0 * ((i as f64 + rt_utils::random()) / SQRT_N as f64) - 1.0;
            let y = 2.0 * ((j as f64 + rt_utils::random()) / SQRT_N as f64) - 1.0;

            if (x.powi(2) + y.powi(2)) < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    println!(
        "Regular estimate of PI: {:.12}",
        (4.0 * inside_circle as f64) / (SQRT_N as f64).powi(2)
    );
    println!(
        "Stratified estimate of PI: {:.12}",
        (4.0 * inside_circle_stratified as f64) / (SQRT_N as f64).powi(2)
    );
}
