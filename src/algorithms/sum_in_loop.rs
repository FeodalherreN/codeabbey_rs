fn sum_in_loop(numbers: Vec<i32>) -> i32 {
    let mut result = 0;
    for number in numbers.iter() {
        result += number;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_in_loop_sums_all_numbers_in_vector() {
        assert_eq!(126, sum_in_loop(vec![10, 20, 30, 40, 5, 6, 7, 8]));
    }
}
