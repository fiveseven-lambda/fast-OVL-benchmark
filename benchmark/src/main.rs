use std::time::{Duration, Instant};

const SMALL: &[(u32, u32)] = &[(10, 2), (11, 3), (12, 4), (13, 4), (14, 5), (15, 5)];

fn main() {
    for &(n, k) in SMALL {
        let now = Instant::now();
        let mut count = 0;
        let mut time = Duration::ZERO;
        loop {
            naive_ovl::p_values(n, k);
            let tmp = now.elapsed();
            if tmp.as_secs() >= 10 {
                break;
            }
            time = tmp;
            count += 1;
        }
        println!("{}, {}: {} in {} ms", n, k, count, time.as_millis());
    }
    for &(n, k) in SMALL {
        let now = Instant::now();
        let mut count = 0;
        let mut time = Duration::ZERO;
        loop {
            fast_ovl::p_values(n, k);
            let tmp = now.elapsed();
            if tmp.as_secs() >= 10 {
                break;
            }
            time = tmp;
            count += 1;
        }
        println!("{}, {}: {} in {} ms", n, k, count, time.as_millis());
    }
}
