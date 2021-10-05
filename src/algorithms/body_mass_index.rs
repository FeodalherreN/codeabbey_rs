fn body_mass_index(input: Vec<Vec<f32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for stats in input {
        let bmi = stats[0] / f32::powf(stats[1], 2.0);

        if bmi < 18.5 {
            answer.push("under".to_string());
        } else if bmi < 25.0 {
            answer.push("normal".to_string());
        } else if bmi < 30.0 {
            answer.push("over".to_string());
        } else {
            answer.push("obese".to_string());
        }
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn body_mass_index_calculates_bmi_and_returns_string_of_results() {
        assert_eq!(
            "over normal under",
            body_mass_index(vec![vec![80.0, 1.73], vec![55.0, 1.58], vec![49.0, 1.91]])
        );
    }
}
