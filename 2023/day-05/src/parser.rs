use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::error::Error;

type Seeds = Vec<u32>;

pub type DestSrcRng = (u32, u32, u32);
pub type Map = Vec<DestSrcRng>;
pub type Maps = Vec<Map>;

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<u32>())(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = take_until(":")(input)?;

    let (input, _) = tag(":\n")(input)?;

    let (input, map_items) = many1(tuple((
        parse_u32,
        tag(" "),
        parse_u32,
        tag(" "),
        parse_u32,
        line_ending,
    )))(input)?;

    Ok((
        input,
        map_items
            .into_iter()
            .map(|(a, _, b, _, c, _)| (a, b, c))
            .collect(),
    ))
}

fn parse_maps(input: &str) -> IResult<&str, Maps> {
    many1(parse_map)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    preceded(tag("seeds: "), separated_list1(space1, parse_u32))(input)
}

fn main_parser(input: &str) -> IResult<&str, (Seeds, Maps)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, maps) = parse_maps(input)?;

    Ok((input, (seeds, maps)))
}

pub fn parse(input: &str) -> Result<(Seeds, Maps), Box<dyn Error>> {
    let (_, res) = main_parser(input).expect("should be able to parse");

    Ok(res)
}
