fn min_of_two(number_pairs_matrix: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for number_pair in number_pairs_matrix.iter() {
        if number_pair.len() < 2 {
            continue;
        }

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
    fn min_of_two_returns_string_with_space_seperated_lowest_numbers_of_pairs() {
        assert_eq!(
            "3 2 15",
            min_of_two(vec![vec![3], vec![5, 3], vec![2, 8], vec![100, 15],])
        );
    }
}
