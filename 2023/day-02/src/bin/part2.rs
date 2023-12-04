use std::cmp;

use day_02::parser::{parse, CubeColor};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(&output);
}

fn part2(input: &str) -> String {
    let games = parse(input).unwrap();

    let res = games
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for set in game.sets.iter() {
                for cube in set.cubes.iter() {
                    match cube.color {
                        CubeColor::Red => max_red = cmp::max(max_red, cube.count),
                        CubeColor::Green => max_green = cmp::max(max_green, cube.count),
                        CubeColor::Blue => max_blue = cmp::max(max_blue, cube.count),
                    };
                }
            }

            max_red * max_green * max_blue
        })
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, "2286".to_string());
    }
}

