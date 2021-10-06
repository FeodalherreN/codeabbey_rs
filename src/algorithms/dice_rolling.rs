fn dice_rolling(random_floats: Vec<f32>) -> String {
    let mut answer: Vec<String> = Vec::new();
    for float in random_floats.iter() {
        let random = (float * 6.0 + 1.0) as i32;
        answer.push(random.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dice_rolling_creates_numbers_from_random_floats() {
        assert_eq!(
            "4 6 3 2 5 1",
            dice_rolling(vec![
                0.59558786964,
                0.861037873663,
                0.385597702116,
                0.246237673331,
                0.808033385314,
                0.0544673665427,
            ]),
        )
    }
}
