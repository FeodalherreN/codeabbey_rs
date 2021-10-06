fn array_counters(array: Vec<i32>) -> String {
    let mut occurences_of_one = 0;
    let mut occurences_of_two = 0;
    let mut occurences_of_three = 0;

    for number in array.iter() {
        match number {
            1 => occurences_of_one += 1,
            2 => occurences_of_two += 1,
            3 => occurences_of_three += 1,
            _ => println!("Not valid input"),
        }
    }

    format!(
        "{} {} {}",
        occurences_of_one, occurences_of_two, occurences_of_three
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_counters_counts_occurences_in_array() {
        assert_eq!("5 2 3", array_counters(vec![3, 2, 1, 2, 3, 1, 1, 1, 1, 3]))
    }
}
