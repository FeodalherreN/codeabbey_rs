fn average_of_an_array(input: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();
    for number_sequence in input.iter() {
        let mut borrowed_number_sequence = number_sequence.to_owned();
        borrowed_number_sequence.retain(|&x| x != 0);
        let sum: i32 = borrowed_number_sequence.iter().sum();
        let average = sum / borrowed_number_sequence.len() as i32;
        answer.push(average.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn average_of_an_array_calculates_average_of_each_vector_of_numbers() {
        assert_eq!(
            "4 15 1",
            average_of_an_array(vec![vec![2, 3, 7, 0], vec![20, 10, 0], vec![1, 0]])
        );
    }
}
