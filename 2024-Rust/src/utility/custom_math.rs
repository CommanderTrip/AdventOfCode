pub fn gcd(a: u128, b: u128) -> u128 {
    if a == b {
        return a;
    }

    if a > b {
        return gcd(a - b, b);
    } else {
        return gcd(a, b - a);
    }
}

#[test]
fn test() {
    assert_eq!(gcd(48, 18), 6)
}
