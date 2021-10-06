fn fahrenheit_to_celcius(numbers: Vec<i32>) -> String {
    let mut answer: Vec<String> = Vec::new();
    let celcius_boiling_degree = 100 as f32;
    let celcius_freezing_degree = 0 as f32;
    let fahrenheit_boiling_degree = 212 as f32;
    let fahrenheit_freezing_degree = 32 as f32;
    let fahrenheits_per_celcius = (fahrenheit_boiling_degree - fahrenheit_freezing_degree)
        / (celcius_boiling_degree - celcius_freezing_degree);

    for number in numbers.iter() {
        let celcius =
            (*number as f32 - fahrenheit_freezing_degree) / fahrenheits_per_celcius as f32;
        let celcius_rounded = celcius.round().to_string();

        answer.push(celcius_rounded.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fahrenheit_to_celcius_converts_fahrenheit_to_celcius() {
        assert_eq!(
            "257 178 76 -39 -6",
            fahrenheit_to_celcius(vec![495, 353, 168, -39, 22])
        );
    }
}
