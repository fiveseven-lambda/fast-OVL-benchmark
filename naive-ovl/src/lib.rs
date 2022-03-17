use itertools::Itertools;

pub fn p_values(n: u32, k: u32) -> f64 {
    let un = n as usize;
    let mut count = 0;
    let mut total = 0;
    for choice in (0..un * 2).combinations(un) {
        let mut seq = vec![false; un * 2];
        for &i in &choice {
            seq[i] = true;
        }
        let mut delta = 0i32;
        let mut delta_max = 0;
        let mut delta_min = 0;
        for &g in &seq {
            delta += if g { 1 } else { -1 };
            delta_max = delta_max.max(delta);
            delta_min = delta_min.min(delta);
        }
        let rho = n - (delta_max - delta_min) as u32;
        if rho <= k {
            count += 1;
        }
        total += 1;
    }
    count as f64 / total as f64
}
