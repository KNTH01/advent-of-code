use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn nums(input: &str) -> IResult<&str, u64> {
    nom::combinator::map(
        preceded(is_not("0123456789"), separated_list1(space1, digit1)),
        |vec| vec.join("").parse::<u64>().unwrap(),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

fn process(input: &str) -> String {
    let (_input, (time, distance)) = parse(input).expect("should parse");

    (0..time)
        .filter_map(|speed| {
            let my_distance = (time - speed) * speed;
            (my_distance > distance).then_some(my_distance)
        })
        .count()
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

        assert_eq!(result, "71503".to_string());
    }
}
