use rtiow::{interval::Interval, utils as rt_utils};

const N: i32 = 100_000;

fn main() {
    let range = Interval::new(-1.0, 1.0);
    let inside_circle = (0..N)
        .into_iter()
        .map(|_| {
            let x = rt_utils::random_in_range(&range);
            let y = rt_utils::random_in_range(&range);

            if (x.powi(2) + y.powi(2)) < 1.0 {
                1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!(
        "Estimate of PI: {}",
        (4.0 * inside_circle as f64) / N as f64
    );
}
