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
    let input = include_str!("./input1.txt");
    let output = process(input);

    dbg!(&output);
}

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

fn process(input: &str) -> String {
    let (_input, (times, distances)) = parse(input).expect("should be valid");

    zip(times, distances)
        .map(|(time, record_distance)| {
            (0..time)
                .filter_map(|speed| {
                    let distance = (time - speed) * speed;
                    (distance > record_distance).then_some(distance)
                })
                .count() as u32
        })
        .product::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );

        assert_eq!(result, "288".to_string());
    }
}
