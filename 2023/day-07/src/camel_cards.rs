use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardLabel {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl CardLabel {
    pub fn value(&self) -> u32 {
        match *self {
            CardLabel::Ace => 14,
            CardLabel::King => 13,
            CardLabel::Queen => 12,
            CardLabel::Jack => 11,
            CardLabel::Ten => 10,
            CardLabel::Nine => 9,
            CardLabel::Eight => 8,
            CardLabel::Seven => 7,
            CardLabel::Six => 6,
            CardLabel::Five => 5,
            CardLabel::Four => 4,
            CardLabel::Three => 3,
            CardLabel::Two => 2,
        }
    }
}

#[derive(Debug)]
pub enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn value(&self) -> u32 {
        match *self {
            HandType::Five => 7,
            HandType::Four => 6,
            HandType::FullHouse => 5,
            HandType::Three => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

type CountCards = HashMap<CardLabel, u32>;
type Cards = Vec<CardLabel>;

#[derive(Debug)]
pub struct Hand {
    pub cards: Cards,
    pub count_cards: HashMap<CardLabel, u32>,
    pub hand_type: HandType,
    pub bid: u32,
}

impl Hand {
    pub fn new(hand: &str, bid: u32) -> Self {
        let mut count_cards: CountCards = HashMap::new();
        let mut cards: Cards = Vec::new();

        for ch in hand.chars() {
            let card = match ch {
                'K' => CardLabel::King,
                'Q' => CardLabel::Queen,
                'J' => CardLabel::Jack,
                'T' => CardLabel::Ten,
                '9' => CardLabel::Nine,
                '8' => CardLabel::Eight,
                '7' => CardLabel::Seven,
                '6' => CardLabel::Six,
                '5' => CardLabel::Five,
                '4' => CardLabel::Four,
                '3' => CardLabel::Three,
                '2' => CardLabel::Two,
                'A' => CardLabel::Ace,
                _ => continue,
            };

            *count_cards.entry(card).or_insert(0) += 1;
            cards.push(card);
        }

        Hand {
            cards,
            count_cards: count_cards.clone(),
            bid,
            hand_type: Hand::compute_hand_type(&count_cards),
        }
    }
}

impl Hand {
    fn compute_hand_type(cards: &CountCards) -> HandType {
        let card_counts = cards
            .iter()
            .map(|(card, count)| *count)
            .filter(|count| *count > 0)
            .collect::<Vec<_>>();

        let max_count = *(card_counts.iter().max().unwrap());

        match max_count {
            5 => HandType::Five,
            4 => HandType::Four,
            3 => {
                if let Some(pair) = card_counts.iter().find(|count| **count == 2) {
                    HandType::FullHouse
                } else {
                    HandType::Three
                }
            }
            2 => {
                if card_counts.iter().filter(|count| **count == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}
