fn arithmetic_progression(formulas: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for formula in formulas.iter() {
        let initial_value = formula[0];
        let step_size = formula[1];
        let iterations = formula[2];
        let mut value = initial_value.to_owned();

        for i in 1..iterations {
            value = value + initial_value + (step_size * i);
        }

        answer.push(value.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_progression_calculation() {
        assert_eq!(
            "21 30",
            arithmetic_progression(vec![vec![5, 2, 3], vec![3, 0, 10]])
        );
    }
}
