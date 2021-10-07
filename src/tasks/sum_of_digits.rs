fn sum_of_digits(digits: Vec<Vec<i32>>) -> String {
    const RADIX: u32 = 10;
    let mut answer: Vec<String> = Vec::new();
    for set_of_digits in digits.iter() {
        let mut inner_result = 0;
        let result_of_digits = set_of_digits[0] * set_of_digits[1] + set_of_digits[2];

        for number_as_char in result_of_digits.to_string().chars() {
            let number_as_digit = number_as_char.to_digit(RADIX).unwrap();
            inner_result = inner_result + number_as_digit;
        }

        answer.push(inner_result.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_digits_calculates_sum_of_given_vector_and_sums_number_by_number() {
        assert_eq!(
            "1 16 21",
            sum_of_digits(vec![vec![11, 9, 1], vec![14, 90, 232], vec![111, 15, 111]])
        );
    }
}
