fn weighted_sum_of_digits(input: Vec<i32>) -> String {
    const RADIX: u32 = 10;
    let mut answer: Vec<String> = Vec::new();
    for value in input.iter() {
        let mut result = 0;
        for (index, digit) in value.to_string().chars().enumerate() {
            let number_as_digit = digit.to_digit(RADIX).unwrap();
            result = result + (number_as_digit * (index as u32 + 1 as u32));
        }

        answer.push(result.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn weighted_sum_of_digits_multiplies_and_adds_numbers_per_index() {
        assert_eq!("9 11 60", weighted_sum_of_digits(vec![9, 15, 1776]));
    }
}
