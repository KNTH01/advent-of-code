use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, multispace0, multispace1, space0, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub set_winning: HashSet<u32>,
    pub set_owned: HashSet<u32>,
}

fn set_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(multispace1, digit1)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(tuple((tag("Card"), multispace1)), digit1)(input)?;

    let (input, sets) = preceded(
        tag(": "),
        preceded(
            multispace0,
            tuple((set_list, multispace1, tag("|"), multispace1, set_list)),
        ),
    )(input)?;

    let sets_to_hashset = |set: Vec<&str>| {
        set.iter()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>()
    };

    let (set_winning, _, _, _, set_owned) = sets;
    let set_winning: HashSet<u32> = sets_to_hashset(set_winning);
    let set_owned: HashSet<u32> = sets_to_hashset(set_owned);

    Ok((
        input,
        Card {
            id: id.parse::<u32>().unwrap(),
            set_winning,
            set_owned,
        },
    ))
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;

    Ok((input, cards))
}

pub fn parse(input: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let (_, cards) = cards(input).expect("should be able to parse input");

    Ok(cards)
}
