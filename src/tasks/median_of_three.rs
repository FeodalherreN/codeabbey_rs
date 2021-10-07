fn median_of_three(list_of_values: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for values in list_of_values.iter() {
        let mut internal_value = values.to_owned();
        internal_value.sort();
        internal_value.reverse();
        answer.push(internal_value[1].to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn median_of_three_returns_median_of_each_vector_given() {
        assert_eq!(
            "5 20 300",
            median_of_three(vec![vec![7, 3, 5], vec![15, 20, 40], vec![300, 550, 137]])
        );
    }
}
