fn main() {
    for _ in 0..1000_000_000 {
        fast_ovl::p_values();
        naive_ovl::p_values();
    }
}
