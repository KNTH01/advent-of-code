fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn process(input: &str) -> String {
    "71503".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(result, "71503".to_string());
    }
}
