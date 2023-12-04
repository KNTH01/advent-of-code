use day_04::parser::Card;
use rayon::prelude::*;

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

fn count_copy_card(orig_cards: &Vec<Card>, copied_cards: &[Card]) -> u32 {
    copied_cards
        .par_iter()
        .map(|card| {
            let match_count = count_winning_cards(card);

            if match_count == 0 {
                return 0;
            }

            let copied_cards = copy_cards(orig_cards, card.id as usize, match_count as usize);

            match_count + count_copy_card(orig_cards, &copied_cards)
        })
        .sum::<u32>()
}

fn process(input: &str) -> String {
    let cards = day_04::parser::parse(input).unwrap();

    let res = cards
        .par_iter()
        .enumerate()
        .map(|(_i, card)| {
            let match_count = count_winning_cards(card);
            let count_copy = &count_copy_card(
                &cards,
                &copy_cards(&cards, card.id as usize, match_count as usize),
            );

            match_count + count_copy + 1
        })
        .sum::<u32>();

    res.to_string()
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

        assert_eq!(result, "30".to_string());
    }
}
