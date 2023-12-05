fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(&output);
}

fn process(input: &str) -> String {
    let res = input;

    // je fais mon algo blablabla

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(result, "142".to_string());
    }
}
