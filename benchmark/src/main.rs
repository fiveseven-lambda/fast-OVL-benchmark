const SMALL: &[(u32, u32)] = &[(10, 2), (11, 3), (12, 4), (13, 4), (14, 5), (15, 5)];
const LARGE: &[(u32, u32)] = &[
    (100, 75),
    (500, 445),
    (1000, 922),
    (5000, 4825),
    (10000, 9753),
];

fn main() {
    for &(n, k) in SMALL.iter().chain(LARGE) {
        println!("{}, {}: {}", n, k, fast_ovl::p_value(n, k));
        println!("{}, {}: {}", n, k + 1, fast_ovl::p_value(n, k + 1));
    }

    for n in 1..=10 {
        for k in 0..=n {
            let naive = naive_ovl::p_value(n, k);
            let fast = fast_ovl::p_value(n, k);
            println!("{}, {}", naive, fast);
            assert_eq!(naive, fast);
        }
    }
    for &(n, k) in SMALL {
        println!("{}, {}: {} sec", n, k, time(naive_ovl::p_value, n, k, 10));
    }
    for &(n, k) in SMALL {
        println!("{}, {}: {} sec", n, k, time(fast_ovl::p_value, n, k, 100000));
    }
    for &(n, k) in LARGE {
        println!("{}, {}: {} sec", n, k, time(fast_ovl::p_value, n, k, 10));
    }
}

fn find_k(n: u32) -> u32 {
    let mut l = 0;
    let mut r = n;
    while l + 1 < r {
        let mid = (l + r) / 2;
        if fast_ovl::p_value(n, mid) <= 0.05 {
            l = mid;
        } else {
            r = mid;
        }
        println!("{} {}", l, r);
    }
    l
}

fn time(fnc: fn(u32, u32) -> f64, n: u32, k: u32, repeat: u32) -> f64 {
    let now = std::time::Instant::now();
    for _ in 0..repeat {
        fnc(n, k);
    }
    now.elapsed().as_secs_f64() / repeat as f64
}
