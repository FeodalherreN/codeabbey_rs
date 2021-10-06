fn reverse_string(input: String) -> String {
    input.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_string_reverses_a_given_string() {
        assert_eq!(
            "oga sraey neves dna erocs ruof",
            reverse_string("four score and seven years ago".to_string())
        )
    }
}
