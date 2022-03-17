use std::time::Instant;

const SMALL: &[(u32, u32)] = &[(10, 2), (11, 3), (12, 4), (13, 4), (14, 5), (15, 5)];

fn main() {
    for &(n, k) in SMALL {
        let repeat = 1;
        let now = Instant::now();
        for _ in 0..repeat {
            naive_ovl::p_value(n, k);
        }
        let duration = now.elapsed();
        println!("{}, {}: {} ms / {} times", n, k, duration.as_millis(), repeat);
    }
    for &(n, k) in SMALL {
        let repeat = 10000;
        let now = Instant::now();
        for _ in 0..repeat {
            fast_ovl::p_value(n, k);
        }
        let duration = now.elapsed();
        println!("{}, {}: {} ms / {} times", n, k, duration.as_millis(), repeat);
    }
}
