fn triangles(input: Vec<Vec<i32>>) -> String {
    let mut answer: Vec<String> = Vec::new();

    for mut triangle in input {
        triangle.sort();

        let is_possible_to_create_triangle = (triangle[0] + triangle[1]) > triangle[2];
        if is_possible_to_create_triangle {
            answer.push("1".to_string());
        } else {
            answer.push("0".to_string());
        }
    }

    answer.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangles_returns_1_if_possible_to_create_0_if_not() {
        assert_eq!("1 0", triangles(vec![vec![3, 4, 5], vec![1, 2, 4]]));
    }
}
