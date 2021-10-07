fn rounding(number_matrix: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for number_pair in number_matrix {
        let divided = number_pair[0] as f32 / number_pair[1] as f32;
        answer.push(divided.round().to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rounding_divides_pairs_and_returns_result_rounded_to_nearest_integer() {
        assert_eq!(
            "2 -4 80",
            rounding(vec![vec![12, 8], vec![11, -3], vec![400, 5]])
        );
    }
}
