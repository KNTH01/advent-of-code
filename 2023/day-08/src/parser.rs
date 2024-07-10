use std::{collections::HashMap, error::Error};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn map_entry(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, (key, (left, right))) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)?;

    Ok((input, (key, (left, right))))
}

fn maps(input: &str) -> IResult<&str, Map> {
    let (input, maps) = separated_list1(line_ending, map_entry)(input)?;

    let maps = maps.iter().copied().collect::<Map>();

    Ok((input, maps))
}

fn parse_commands(input: &str) -> IResult<&str, (&str, Map)> {
    let (input, commands) = terminated(alpha1, line_ending)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, maps) = maps(input)?;

    Ok((input, (commands, maps)))
}

pub fn parse(input: &str) -> Result<(&str, Map), Box<dyn Error>> {
    let (_input, res) = parse_commands(input).expect("should parse");

    Ok(res)
}
