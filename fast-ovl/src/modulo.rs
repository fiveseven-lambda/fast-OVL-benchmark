/// returns `a` + `b` modulo `modulus`.
///
/// requirement: `a`, `b` < `modulus`.
pub fn add(a: u32, b: u32, modulus: u32) -> u32 {
    if a < modulus - b {
        a + b
    } else {
        a - (modulus - b)
    }
}

/// returns `a` - `b` modulo `modulus`.
///
/// requirement: `a`, `b` < `modulus`.
pub fn sub(a: u32, b: u32, modulus: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        a + modulus - b
    }
}

pub fn mul(a: u32, b: u32, modulus: u32) -> u32 {
    (a as u64 * b as u64 % modulus as u64) as u32
}

pub fn div(a: u32, b: u32, modulus: u32) -> u32 {
    mul(a, inv(b, modulus), modulus)
}

pub fn pow(mut x: u32, mut n: u32, modulus: u32) -> u32 {
    let mut ret = 1;
    while n != 0 {
        if n % 2 != 0 {
            mul_assign(x, &mut ret, modulus);
        }
        mul_assign(x, &mut x, modulus);
        n /= 2;
    }
    ret
}

#[test]
fn test_pow() {
    for modulus in [1_000_000_007] {
        assert_eq!(pow(0, 0, modulus), 1);
        assert_eq!(pow(0, 1, modulus), 0);
        assert_eq!(pow(0, 2, modulus), 0);

        for x in 1..modulus.min(100) {
            let mut ans = 1;
            for n in 0..100 {
                assert_eq!(pow(x, n, modulus), ans);
                mul_assign(x, &mut ans, modulus);
            }
        }
    }
}

pub fn inv(x: u32, modulus: u32) -> u32 {
    let mut a = (modulus, 0);
    let mut b = (x, 1);
    let mut sign = false;
    while b.0 != 0 {
        a.1 += b.1 * (a.0 / b.0);
        a.0 %= b.0;
        sign = !sign;
        std::mem::swap(&mut a, &mut b);
    }
    if sign {
        a.1
    } else {
        modulus - a.1
    }
}

#[test]
fn test_inv() {
    let n = 1000;
    for modulus in [
        2,
        3,
        5,
        7,
        11,
        13,
        17,
        19,
        23,
        29,
        998_244_353,
        1_000_000_007,
        4_294_967_291,
        4_294_967_279,
        4_294_967_231,
        4_294_967_197,
        4_294_967_189,
        4_294_967_161,
        4_294_967_143,
        4_294_967_111,
        4_294_967_087,
        4_294_967_029,
    ] {
        for n in 1..modulus.min(n + 1) {
            let inv = inv(n, modulus);
            assert_eq!(mul(n, inv, modulus), 1);
        }
        for n in modulus.max(n + 1) - n..modulus {
            let inv = inv(n, modulus);
            assert_eq!(mul(n, inv, modulus), 1);
        }
    }
}

// pub fn add_assign(src: u32, dest: &mut u32, modulus: u32) {
//     *dest = add(*dest, src, modulus);
// }

// pub fn sub_assign(src: u32, dest: &mut u32, modulus: u32) {
//     *dest = sub(*dest, src, modulus);
// }

pub fn mul_assign(src: u32, dest: &mut u32, modulus: u32) {
    *dest = mul(*dest, src, modulus);
}

pub fn div_assign(src: u32, dest: &mut u32, modulus: u32) {
    *dest = div(*dest, src, modulus);
}
