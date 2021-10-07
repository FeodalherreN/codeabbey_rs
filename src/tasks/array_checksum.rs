fn array_checksum(input: Vec<i32>) -> String {
    const SEED: i32 = 113;
    const LIMIT: i32 = 10000007;
    let mut result = 0;

    for number in input.iter() {
        result = (result + number) * SEED;
        if result > LIMIT {
            result = result % LIMIT;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_checksum_calculates_checksum() {
        assert_eq!("8921379", array_checksum(vec![3, 1, 4, 1, 5, 9]))
    }
}
