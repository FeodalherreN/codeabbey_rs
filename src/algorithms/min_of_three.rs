fn min_of_three(number_pairs_matrix: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for number_pair in number_pairs_matrix.iter() {
        let mut lowest_number = 0;
        for number in number_pair.iter() {
            if lowest_number.to_owned() == 0 || number < &lowest_number {
                lowest_number = number.to_owned();
            }
        }

        answer.push(lowest_number.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_of_three_returns_string_with_space_seperated_lowest_numbers_of_three() {
        assert_eq!(
            "3 15 137",
            min_of_three(vec![vec![7, 3, 5], vec![15, 20, 40], vec![300, 550, 137],])
        );
    }
}
