fn collatz_sequence(numbers: Vec<i32>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for number in numbers {
        answer.push(calculate(number, 0).to_string());
    }

    answer.join(" ")
}

fn calculate(number: i32, count: i32) -> i32 {
    let mut borrowed_count = count.to_owned();
    let mut borrowed_number = number.to_owned();
    borrowed_count = borrowed_count + 1;

    if number % 2 == 0 {
        borrowed_number = borrowed_number / 2;
    } else {
        borrowed_number = 3 * borrowed_number + 1;
    }

    if borrowed_number != 1 {
        borrowed_count = calculate(borrowed_number, borrowed_count);
    }

    borrowed_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collatz_sequence_calculates_number_of_calculation_it_does_before_reaching_one() {
        assert_eq!("1 17 118", collatz_sequence(vec![2, 15, 97]))
    }
}
