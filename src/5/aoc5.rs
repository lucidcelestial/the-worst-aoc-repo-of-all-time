use std::{collections::HashMap, ops::Range};

// literally just doing this to reduce clutter
trait CanLookup {
    fn look_up(&self, value: &u64) -> u64;
}

impl CanLookup for Vec<(Range<u64>, Range<u64>)> {
    fn look_up(&self, value: &u64) -> u64 {
        for tuple in self {
            if tuple.0.contains(value) {
                return tuple.1.start + (*value - tuple.0.start);
            }
        }
        *value
    }
}

fn parse_to_ranges(input: &str) -> Vec<(Range<u64>, Range<u64>)> {
    input
        .split("\n")
        .skip(1)
        .map(|x| {
            x.split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|x| {
            let dest = x.get(0).unwrap();
            let src = x.get(1).unwrap();
            let n = x.get(2).unwrap();

            (src.to_owned()..(src + n), dest.to_owned()..(dest + n))
        })
        .collect()
}

fn main() {
    let categories = ["soil", "fert", "water", "light", "temp", "humid", "loc"];

    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut split = file.split("\n\n");
    let seeds = split.next().unwrap();

    let maps = categories
        .iter()
        .map(|category| (*category, parse_to_ranges(split.next().unwrap())))
        .collect::<HashMap<&str, Vec<(Range<u64>, Range<u64>)>>>();

    let min = seeds
        .replace("seeds: ", "")
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .map(|seed| {
            let mut value = seed;

            for category in categories {
                value = maps.get(category).unwrap().look_up(&value);
            }

            value
        })
        .min()
        .unwrap();

    print!("{}", min);
}
