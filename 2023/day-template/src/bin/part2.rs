fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn process(input: &str) -> String {
    "1234".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("1234");

        assert_eq!(result, "1234".to_string());
    }
}

