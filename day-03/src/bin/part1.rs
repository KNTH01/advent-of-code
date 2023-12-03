use day_03::map::Map;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(&output);
}

fn part1(input: &str) -> String {
    let map = Map::new(input);
    dbg!(&map);

    "4361".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(result, "4361".to_string());
    }
}
