fn maximum_of_array(numbers: Vec<i32>) -> String {
    let mut lowest_number = 0;
    let mut highest_number = 0;

    for number in numbers.iter() {
        if lowest_number == 0 || &lowest_number > number {
            lowest_number = number.to_owned();
        }

        if highest_number == 0 || &highest_number < number {
            highest_number = number.to_owned();
        }
    }

    let result = format!("{} {}", highest_number, lowest_number);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_of_array_returns_maximum_and_minimum_numbers_of_vector() {
        assert_eq!(
            "300 1",
            maximum_of_array(vec![
                1, 3, 5, 7, 9, 11, 295, 297, 299, 300, 298, 296, 12, 10, 8, 6, 4, 2
            ])
        );
    }
}
