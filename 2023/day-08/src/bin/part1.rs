use day_08::parser::parse;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn process(input: &str) -> String {
    let (commands, map) = parse(input).unwrap();
    dbg!(&map);

    commands.chars().for_each(|c| {
        // match c {}
    });
    "1234".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(result, "6".to_string());
    }
}
