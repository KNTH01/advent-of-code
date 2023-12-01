fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(&output);
}

fn part2(input: &str) -> String {
    let numbers: Vec<_> = (1..=9)
        .collect::<Vec<i32>>()
        .iter()
        .map(|n| n.to_string())
        .collect();

    let alpha_numbers: Vec<String> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|str| str.to_string())
    .collect();

    let lines = input.lines();

    let res = lines
        .map(|l| {
            let mut res = String::new();
            let mut start_index = 0;

            while start_index < l.len() {
                let mut found = false;

                for (i, an) in alpha_numbers.iter().enumerate() {
                    let end_index = start_index + an.len();
                    if end_index <= l.len() && &l[start_index..end_index] == an {
                        res.push_str(&numbers[i]);
                        start_index = end_index;
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    res.push(l.chars().nth(start_index).unwrap());
                    start_index += 1;
                }
            }

            res
        })
        .map(|l| {
            let first_digit = l
                .chars()
                .find(|c| numbers.contains(&c.to_string()))
                .unwrap()
                .to_string();

            let last_digit = l
                .chars()
                .rev()
                .find(|c| numbers.contains(&c.to_string()))
                .unwrap()
                .to_string();

            let digits = format!("{first_digit}{last_digit}");

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
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );

        assert_eq!(result, "281".to_string());
    }
}

