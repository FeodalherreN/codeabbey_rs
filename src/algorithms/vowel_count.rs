fn vowel_count(scentences: Vec<String>) -> String {
    const VOWELS: &str = "aouiey";
    let mut answer: Vec<String> = Vec::new();

    for scentence in scentences {
        let mut vowel_count = 0;
        for (_index, character) in scentence.chars().enumerate() {
            if VOWELS.contains(character) {
                vowel_count = vowel_count + 1;
            }
        }
        answer.push(vowel_count.to_string());
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vowel_count_counts_vowels_and_returns_result_as_space_seperated_string() {
        assert_eq!(
            "5 4 13 2",
            vowel_count(vec![
                "abracadabra".to_string(),
                "pear tree".to_string(),
                "o a kak ushakov lil vo kashu kakao".to_string(),
                "my pyx".to_string()
            ])
        );
    }
}
