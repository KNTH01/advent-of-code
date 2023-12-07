use std::iter::zip;

use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn parse() {}

fn process(input: &str) -> String {}

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

        assert_eq!(result, "288".to_string());
    }
}
