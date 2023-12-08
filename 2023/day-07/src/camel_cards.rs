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
    pub fn value(&self, with_joker: bool) -> u32 {
        match *self {
            CardLabel::Ace => 14,
            CardLabel::King => 13,
            CardLabel::Queen => 12,
            CardLabel::Jack => {
                if with_joker {
                    1
                } else {
                    11
                }
            }
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

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
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
            HandType::TwoPairs => 3,
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
    pub fn new(hand: &str, bid: u32, with_joker: bool) -> Self {
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
            hand_type: Hand::compute_hand_type(&count_cards, with_joker),
        }
    }
}

impl Hand {
    fn compute_hand_type(cards: &CountCards, with_joker: bool) -> HandType {
        let count_jokers = if with_joker {
            *(cards.get(&CardLabel::Jack).unwrap_or(&0))
        } else {
            0
        };

        let count_cards = cards
            .iter()
            .map(|(_card, count)| *count)
            .filter(|count| *count > 0)
            .collect::<Vec<_>>();

        let max_count = *(count_cards.iter().max().unwrap());

        match max_count {
            5 => HandType::Five,
            4 => {
                if count_jokers > 0 {
                    HandType::Five
                } else {
                    HandType::Four
                }
            }
            3 => {
                let has_pair = count_cards.iter().find(|count| **count == 2);

                match count_jokers {
                    0 => {
                        if has_pair.is_some() {
                            HandType::FullHouse
                        } else {
                            HandType::Three
                        }
                    }
                    1 => HandType::Four,
                    _ => HandType::Five,
                }
            }
            2 => {
                let has_two_pairs = count_cards.iter().filter(|count| **count == 2).count() == 2;

                match count_jokers {
                    0 => {
                        if has_two_pairs {
                            HandType::TwoPairs
                        } else {
                            HandType::OnePair
                        }
                    }
                    1 => {
                        if has_two_pairs {
                            HandType::FullHouse
                        } else {
                            HandType::Three
                        }
                    }
                    2 => HandType::Four,
                    _ => HandType::Five,
                }
            }
            _ => match count_jokers {
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            },
        }
    }
}
