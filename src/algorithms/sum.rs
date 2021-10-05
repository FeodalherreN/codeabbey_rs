fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_sums_two_integers() {
        assert_eq!(8, sum(5, 3));
    }
}
