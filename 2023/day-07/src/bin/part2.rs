use std::cmp::Ordering;

use day_07::camel_cards::Hand;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn process(input: &str) -> String {
    let use_joker = true;
    
    let res = input
        .lines()
        .filter_map(|line| {
            let [hand, bid] = line.split(' ').collect::<Vec<&str>>()[..] else {
                return None;
            };

            Some(Hand::new(hand, bid.parse::<u32>().unwrap(), use_joker))
        })
        .sorted_by(|a, b| match b.hand_type.value().cmp(&a.hand_type.value()) {
            Ordering::Equal => {
                let a: Vec<u32> = a.cards.iter().map(|c| c.value(use_joker)).collect();
                let b: Vec<u32> = b.cards.iter().map(|c| c.value(use_joker)).collect();

                for (b, a) in b.iter().zip(a.iter()) {
                    match b.cmp(a) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        })
        .collect::<Vec<_>>();

    let count_hands = res.len();

    res.iter()
        .enumerate()
        .fold(0, |acc, (i, card)| {
            let res = (count_hands - i) as u32 * card.bid;
            acc + res
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );

        assert_eq!(result, "5905".to_string());
    }
}

