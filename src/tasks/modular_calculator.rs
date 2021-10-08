use std::collections::HashMap;

fn modular_calculator(start_number: i32, operations: Vec<(&str, i32)>) -> i32 {
    let mut answer = start_number.to_owned();
    for (operation, number) in operations {
        let s = answer.to_owned();
        match operation {
            "+" => answer = answer + number,
            "*" => answer = answer * number,
            "%" => answer = answer % number,
            _ => (),
        };

        println!(
            "Before: {} operation: {} {} after: {}",
            s, operation, number, answer
        );
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modular_calculator_calculates_operations_in_input_and_returns_result() {
        assert_eq!(
            1,
            modular_calculator(
                5,
                [
                    ("+", 3),
                    ("*", 7),
                    ("+", 10),
                    ("*", 2),
                    ("*", 3),
                    ("+", 1),
                    ("%", 11)
                ]
                .iter()
                .cloned()
                .collect()
            )
        )
    }
}
