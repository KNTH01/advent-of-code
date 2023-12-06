use day_05::parser::parse;
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    dbg!(&output);
}

fn process(input: &str) -> String {
    let res = parse(input).unwrap();

    let (seeds, maps) = res;

    let seeds = seeds
        .chunks_exact(2)
        .flat_map(|chunk| {
            let a = chunk[0];
            let b = chunk[1];

            (a..(a + b)).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .into_par_iter()
        .map(|seed| {
            let res = maps.iter().fold(seed, |mapped, map| {
                let res_rng = map
                    .iter()
                    .map(|(dest, src, rng)| {
                        let dest_rng = *dest..*dest + *rng;
                        let src_rng = *src..*src + *rng;

                        (dest_rng, src_rng)
                    })
                    .find(|(_, src_rng)| src_rng.contains(&mapped));

                if let Some((dest_rng, src_rng)) = res_rng {
                    dest_rng.start + (mapped - src_rng.start)
                } else {
                    mapped
                }
            });

            // dbg!(&res);

            res
        })
        .min()
        .unwrap()
        .to_string()
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

        assert_eq!(result, "46".to_string());
    }
}
