use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::error::Error;

#[derive(Debug)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Cube {
    pub count: u32,
    pub color: CubeColor,
}

#[derive(Debug)]
pub struct Set {
    pub cubes: Vec<Cube>,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

// 3 blue
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (num, color)) = map_res(
        separated_pair(digit1, tag(" "), alpha1),
        |(num_str, color_str): (&str, &str)| {
            let num = num_str.parse::<u32>().unwrap();

            Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>((
                num,
                match color_str {
                    "red" => CubeColor::Red,
                    "green" => CubeColor::Green,
                    "blue" => CubeColor::Blue,
                    _ => unreachable!(),
                },
            ))
        },
    )(input)?;

    Ok((input, Cube { count: num, color }))
}

// 3 blue, 4 red
fn set(input: &str) -> IResult<&str, Set> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, Set { cubes }))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;

    let (input, sets) = preceded(tag(": "), separated_list1(tag("; "), set))(input)?;

    Ok((
        input,
        Game {
            id: id.parse::<u32>().unwrap(),
            sets,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;

    Ok((input, games))
}

pub fn parse(input: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let (_, games) = parse_games(input).expect("should be able to parse input");

    Ok(games)
}
