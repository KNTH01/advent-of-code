use day_02::parser::{parse, CubeColor};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(&output);
}

fn part1(input: &str) -> String {
    let res = parse(input).unwrap();

    let res = res
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                let mut ok = true;

                for cube in set.cubes.iter() {
                    ok = match cube.color {
                        CubeColor::Red => cube.count <= 12,
                        CubeColor::Green => cube.count <= 13,
                        CubeColor::Blue => cube.count <= 14,
                    };

                    if !ok {
                        break;
                    }
                }
                ok
            })
        })
        .map(|game| game.id)
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, "8".to_string());
    }
}
