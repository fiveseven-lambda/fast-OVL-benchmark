use crate::bit::UnsignedInt;
use crate::modulo;
use crate::number_theoretic_transform::Prime;

/// binom(2n, n) * (1 - p<sub>2,n,n</sub>(k / n))
pub fn p_value_modulo(n: u32, k: u32, prime: &Prime) -> u32 {
    let bit = n.bit_width();
    let un = n as usize;

    let mut p = fibonacci_polynomial(n - k, prime.value());
    let q = fibonacci_polynomial(n - k - 1, prime.value());
    prime.polydiff(&mut p);
    let mut inv_q = prime.polyinv(q, bit);
    inv_q.truncate(un + 1);
    prime.polymul(inv_q, &mut p);

    let mut r = fibonacci_polynomial(n - k + 1, prime.value());
    let s = fibonacci_polynomial(n - k, prime.value());
    prime.polydiff(&mut r);
    let mut inv_s = prime.polyinv(s, bit);
    inv_s.truncate(un + 1);
    prime.polymul(inv_s, &mut r);

    modulo::sub(p[un], r[un], prime.value())
}

/// Fibonacci polynomials
///
/// defined by:
/// - <i>F</i><sub>0</sub>(<i>x</i>) = F<sub>1</sub>(x) = 1
/// - F<sub>n + 2</sub>(x) = F<sub>n + 1</sub>(x) - x F<sub>n</sub>(x)
fn fibonacci_polynomial(n: u32, modulus: u32) -> Vec<u32> {
    (0..=n / 2)
        .scan(1, |t, i| {
            let ret = *t;
            if i < n / 2 {
                modulo::mul_assign(n - i * 2, t, modulus);
                modulo::mul_assign(n - i * 2 - 1, t, modulus);
                modulo::div_assign(i + 1, t, modulus);
                modulo::div_assign(n - i, t, modulus);
            }
            Some(if i % 2 == 0 {
                ret
            } else {
                modulo::sub(0, ret, modulus)
            })
        })
        .collect()
}

#[cfg(test)]
use itertools::Itertools;

#[test]
fn test_p_value_modulo() {
    let prime = Prime::from_coef_exp(119, 23).unwrap();
    assert_eq!(prime.value(), 998244353);
    for n in 1..10 {
        let mut freq = vec![0; n + 1];
        for choice in (0..n * 2).combinations(n) {
            let mut seq = vec![false; n * 2];
            for index in choice {
                seq[index] = true;
            }
            let mut delta = 0i32;
            let mut max_delta = 0;
            let mut min_delta = 0;
            for g in seq {
                delta += if g { 1 } else { -1 };
                max_delta = max_delta.max(delta);
                min_delta = min_delta.min(delta);
            }
            let rho = n - (max_delta - min_delta) as usize;
            freq[rho] += 1;
        }
        for i in (0..n).rev() {
            freq[i] += freq[i + 1];
        }
        for i in 0..n {
            assert_eq!(
                p_value_modulo(n as u32, i as u32, &prime),
                freq[i + 1] % prime.value()
            );
        }
    }
}
