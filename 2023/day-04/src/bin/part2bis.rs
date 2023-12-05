use std::collections::HashMap;

use day_04::parser::Card;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(&output);
}

fn copy_cards(orig_cards: &Vec<Card>, i: usize, n: usize) -> Vec<Card> {
    let end = usize::min(i + n, orig_cards.len());

    orig_cards[i..end].to_vec()
}

fn count_winning_cards(card: &Card) -> u32 {
    let intersection = card
        .set_winning
        .intersection(&card.set_owned)
        .map(|x| *x as u32)
        .collect::<Vec<u32>>();

    intersection.len() as u32
}

fn count_copy_card(hash_map: &CardMapWinningNum, card_id: u32) -> u32 {
    let mut res: u32 = 1;
    let winning_num = hash_map.get(&card_id).unwrap_or(&0);

    if *winning_num == 0 {
        return 0;
    }

    for i in card_id + 1..=(card_id + winning_num + 1) {
        res += count_copy_card(hash_map, i);
    }

    res
}

fn process(input: &str) -> String {
    let cards = day_04::parser::parse(input).unwrap();

    let mut hash_map: CardMapWinningNum = HashMap::new();

    let count_orig = cards
        .iter()
        .map(|card| {
            let match_count = count_winning_cards(card);
            hash_map.insert(card.id, match_count);
            match_count + 1
        })
        .sum::<u32>();

    let count_copies = hash_map
        .keys()
        .map(|card_id| count_copy_card(&hash_map, *card_id))
        .sum::<u32>();

    let res = count_copies + count_orig;

    res.to_string()
}

type CardMapWinningNum = HashMap<u32, u32>;

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

        assert_eq!(result, "30".to_string());
    }
}
