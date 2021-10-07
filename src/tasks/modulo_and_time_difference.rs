fn modulo_and_time_difference(input: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for td in input.iter() {
        let (mut s, mut m, mut h, mut d) = (0, 0, 0, 0);
        let (s1, mut s2, m1, mut m2, h1, mut h2, d1, mut d2) =
            (td[3], td[7], td[2], td[6], td[1], td[5], td[0], td[4]);

        if s2 >= s1 {
            s = s2 - s1;
        } else {
            m2 -= 1;
            s2 += 60;
            s = s2 - s1;
        }

        if m2 >= m1 {
            m = m2 - m1;
        } else {
            h2 -= 1;
            m2 += 60;
            m = m2 - m1;
        }

        if h2 >= h1 {
            h = h2 - h1;
        } else {
            d2 -= 1;
            h2 += 24;
            h = h2 - h1;
        }

        d = d2 - d1;

        answer.push(format!("({} {} {} {})", d, h, m, s));
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modulo_and_time_difference_calculates_difference_in_time() {
        assert_eq!(
            "(1 3 4 5) (19 0 57 23) (1 7 44 26)",
            modulo_and_time_difference(vec![
                vec![1, 0, 0, 0, 2, 3, 4, 5],
                vec![5, 3, 23, 22, 24, 4, 20, 45],
                vec![8, 4, 6, 47, 9, 11, 51, 13]
            ])
        )
    }
}
