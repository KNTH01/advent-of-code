use day_03::map::{Map, Point, TileType};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(&output);
}

#[derive(Debug)]
struct TileNum {
    idx_start: usize,
    idx_end: usize,
    num: String,
}

impl TileNum {
    fn is_part_num(&self, map: &Map) -> bool {
        let p1 = map.map_point(self.idx_start);
        let p2 = map.map_point(self.idx_end);

        let top_left_p = Point {
            x: p1.x - 1,
            y: p1.y - 1,
        };

        let top_right_p = Point {
            x: p2.x + 1,
            y: p2.y - 1,
        };

        let btm_left_p = Point {
            x: p1.x - 1,
            y: p1.y + 1,
        };

        let btm_right_p = Point {
            x: p2.x + 1,
            y: p2.y + 1,
        };

        let left_p = Point {
            x: p1.x - 1,
            y: p1.y,
        };

        let right_p = Point {
            x: p2.x + 1,
            y: p2.y,
        };

        let mut please_check: Vec<_> = vec![
            top_left_p,
            top_right_p,
            btm_left_p,
            btm_right_p,
            left_p,
            right_p,
        ];

        for i in 0..self.num.len() {
            let i = i as i32;

            please_check.push(Point {
                x: p1.x + i,
                y: p1.y - 1,
            });

            please_check.push(Point {
                x: p1.x + i,
                y: p1.y + 1,
            });
        }

        please_check
            .iter()
            .filter(|p| map.in_bounds(p))
            .any(|p| map.get_tile(p).t == TileType::Symbol)
    }
}

fn part2(input: &str) -> String {
    let map = Map::new(input);

    let mut nums = Vec::new();
    let mut buf = String::new();
    let mut idx_start: Option<usize> = None;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if tile.t == TileType::Digit {
            buf.push(tile.c);
            if idx_start.is_none() {
                idx_start = Some(idx)
            }
        } else if !buf.is_empty() {
            nums.push(TileNum {
                idx_start: idx_start.unwrap(),
                idx_end: idx - 1,
                num: buf.clone(),
            });

            buf.clear();
            idx_start = None;
        }
    }

    nums.iter()
        .filter(|n| n.is_part_num(&map))
        .map(|n| n.num.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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

