fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(&output);
}

fn process(input: &str) -> String {
    let res = day_04::parser::parse(input).unwrap();

    res.iter()
        .map(|card| {
            let intersection = card
                .set_winning
                .intersection(&card.set_owned)
                .map(|x| *x as u32)
                .collect::<Vec<u32>>();

            let intersection_len = intersection.len() as u32;

            if intersection_len == 0 {
                return 0;
            }

            let extra_winning_count: u32 = intersection_len - 1;

            if extra_winning_count == 0 {
                1
            } else {
                let res: u32 = 2;

                res.pow(extra_winning_count)
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );

        assert_eq!(result, "13".to_string());
    }
}
