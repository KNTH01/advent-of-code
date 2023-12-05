use day_05::parser::parse;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(&output);
}

fn process(input: &str) -> String {
    let res = parse(input).unwrap();

    let (seeds, maps) = res;

    let mut list = Vec::new();

    for seed in seeds {
        let mut mapped: u128 = seed;

        for (i, map) in maps.iter().enumerate() {
            let mut used_src: u128 = 0;
            let mut used_dest: u128 = 0;
            let mut found_rng: bool = false;

            for (dest, src, rng) in map {
                if (src..=&(src + rng)).contains(&&mapped) {
                    used_src = *src;
                    used_dest = *dest;
                    found_rng = true;
                }
            }

            if found_rng {
                mapped = used_dest + (mapped - used_src);
            }

            println!("seed {seed}, map {i}: {mapped}");
            list.push(mapped);
        }
    }

    list.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );

        assert_eq!(result, "13".to_string());
    }
}
