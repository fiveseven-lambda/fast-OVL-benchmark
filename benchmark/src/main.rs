const SMALL: &[(u32, u32)] = &[
    (10, 5),
    (12, 6),
    (14, 7),
    (16, 8),
];
const LARGE: &[(u32, u32)] = &[
    (100, 50),
    (500, 250),
    (1000, 500),
    (5000, 2500),
    (10000, 5000),
];

fn main() {
    // for n in 1..=10 {
    //     for k in 0..=n {
    //         let naive = naive_ovl::p_value(n, k);
    //         let fast = fast_ovl::p_value(n, k);
    //         println!("{}, {}", naive, fast);
    //         assert_eq!(naive, fast);
    //     }
    // }
    println!("naive OVL");
    for &(n, k) in SMALL {
        println!("n = {}: {} sec", n, time(naive_ovl::p_value, n, k, 10));
    }
    println!("fast OVL");
    for &(n, k) in SMALL {
        println!("n = {}: {} sec", n, time(fast_ovl::p_value, n, k, 100000));
    }
    for &(n, k) in LARGE {
        println!("n = {}: {} sec", n, time(fast_ovl::p_value, n, k, 10));
    }
}

fn time(fnc: fn(u32, u32) -> f64, n: u32, k: u32, repeat: u32) -> f64 {
    let now = std::time::Instant::now();
    for _ in 0..repeat {
        fnc(n, k);
    }
    now.elapsed().as_secs_f64() / repeat as f64
}
