use std::collections::HashSet;

use day_03::map::{Map, Point, TileType};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(&output);
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct TileNum {
    idx_start: usize,
    idx_end: usize,
    num: String,
}

impl TileNum {
    fn get_coords(&self, map: &Map) -> Vec<Point> {
        let mut res = Vec::new();

        for i in self.idx_start..=self.idx_end {
            res.push(map.map_point(i));
        }

        res
    }
}

struct TileGear {
    p: Point,
}

impl TileGear {
    fn check_surrounding_for_gear_ratio(&self, map: &Map, nums: &[TileNum]) -> Option<u32> {
        let p = self.p;

        let top_left_p = Point {
            x: p.x - 1,
            y: p.y - 1,
        };
        let top_right_p = Point {
            x: p.x + 1,
            y: p.y - 1,
        };
        let btm_left_p = Point {
            x: p.x - 1,
            y: p.y + 1,
        };
        let btm_right_p = Point {
            x: p.x + 1,
            y: p.y + 1,
        };
        let top_p = Point { x: p.x, y: p.y - 1 };
        let btm_p = Point { x: p.x, y: p.y + 1 };
        let left_p = Point { x: p.x - 1, y: p.y };
        let right_p = Point { x: p.x + 1, y: p.y };

        let please_check = [
            top_p,
            top_left_p,
            top_right_p,
            btm_p,
            btm_left_p,
            btm_right_p,
            left_p,
            right_p,
        ];

        let mut num_set = HashSet::new();

        please_check
            .iter()
            .filter(|p| map.in_bounds(p))
            .for_each(|p| {
                if let Some(num) = nums
                    .iter()
                    .find(|num| num.get_coords(map).iter().any(|c| c == p))
                {
                    num_set.insert(num);
                }
            });

        let count: u32 = num_set.len().try_into().unwrap();

        if count == 2 {
            Some(
                num_set
                    .iter()
                    .map(|ns| ns.num.parse::<u32>().unwrap())
                    .product(),
            )
        } else {
            None
        }
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

    map.tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| t.t == TileType::Symbol && t.c == '*')
        .map(|(i, _t)| TileGear {
            p: map.map_point(i),
        })
        .filter_map(|tile_gear| tile_gear.check_surrounding_for_gear_ratio(&map, &nums))
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

        assert_eq!(result, "467835".to_string());
    }
}
