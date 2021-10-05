fn sums_in_loops(number_matrix: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();
    for number_list in number_matrix.iter() {
        if number_list.len() < 2 {
            continue;
        }

        let mut list_result = 0;
        for number in number_list.iter() {
            list_result += number;
        }

        answer.push(list_result.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sums_in_loops_iterates_and_sums_numbers_in_lists() {
        assert_eq!(
            "108 260 1999",
            sums_in_loops(vec![vec![100, 8], vec![15, 245], vec![1945, 54]])
        );
    }
}
