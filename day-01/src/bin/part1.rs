fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(&output);
}

fn part1(input: &str) -> String {
    let numbers: Vec<_> = (0..=9)
        .collect::<Vec<i32>>()
        .iter()
        .map(|n| n.to_string())
        .collect();

    let lines = input.lines();

    let res = lines
        .map(|l| {
            let first_digit = l
                .chars()
                .find(|c| numbers.contains(&c.to_string()))
                .unwrap()
                .to_string();

            let second_digit = l
                .chars()
                .rev()
                .find(|c| numbers.contains(&c.to_string()))
                .unwrap()
                .to_string();

            let digits = format!("{first_digit}{second_digit}");

            digits.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(result, "142".to_string());
    }
}
