fn linear_function(input: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();
    for numbers in input {
        let a = numbers[0];
        let b = numbers[1];
        let c = numbers[2];
        let d = numbers[3];

        let slope = (d - b) / (c - a);
        let intercept = b - slope * a;

        answer.push(format!("({} {})", slope, intercept));
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_function_test() {
        assert_eq!(
            "(1 0) (-1 1)",
            linear_function(vec![vec![0, 0, 1, 1], vec![1, 0, 0, 1]])
        )
    }
}
