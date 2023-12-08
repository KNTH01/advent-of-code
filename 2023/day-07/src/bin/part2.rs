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
                let a_val: Vec<u32> = a.cards.iter().map(|c| c.value(use_joker)).collect();
                let b_val: Vec<u32> = b.cards.iter().map(|c| c.value(use_joker)).collect();

                for (b_val, a_val) in b_val.iter().zip(a_val.iter()) {
                    // println!("cmp: {}:{:?}, {}:{:?}", b_val, &b.cards, a_val, &a.cards);
                    match b_val.cmp(a_val) {
                        Ordering::Equal => {
                            continue;
                        }
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
    use day_07::camel_cards::HandType;

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

    #[test]
    fn joker() {
        let use_joker = true;
        let hand_type = |str: &str| Hand::new(str, 0, use_joker).hand_type;

        assert_eq!(HandType::Five, hand_type("AAAAA"));
        assert_eq!(HandType::Five, hand_type("AAAAJ"));
        assert_eq!(HandType::Five, hand_type("AAAJJ"));
        assert_eq!(HandType::Five, hand_type("AAJJJ"));
        assert_eq!(HandType::Five, hand_type("AJJJJ"));
        assert_eq!(HandType::Four, hand_type("AA2JJ"));
        assert_eq!(HandType::Four, hand_type("AAA2J"));
        assert_eq!(HandType::FullHouse, hand_type("AA22J"));
        assert_eq!(HandType::Four, hand_type("AAJ2A"));
        assert_eq!(HandType::Three, hand_type("AAJ23"));
        assert_eq!(HandType::TwoPairs, hand_type("AA223"));
        assert_eq!(HandType::OnePair, hand_type("A234J"));
        assert_eq!(HandType::HighCard, hand_type("A2345"));
    }
}
