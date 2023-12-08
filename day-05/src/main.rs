mod mapping;
mod util;

use crate::mapping::Mappings;

fn main() {
    let input = util::read_input();

    let mut lines = input.lines().filter(|l| !l.is_empty()); //.collect::<Vec<_>>();

    // first line contains the seeds, extract them
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // extract the mappings
    let mapping_lines = lines.collect::<Vec<_>>();
    let mappings = Mappings::from_string(&mapping_lines);

    // part 1
    let mut current_lowest_location = u64::MAX;
    for seed in &seeds {
        current_lowest_location = current_lowest_location.min(chain_map(*seed, &mappings));
    }

    println!("Lowest possible location (part 1): {current_lowest_location}",);

    // part 2
    let mut current_lowest_location = u64::MAX;
    for seed_range in seeds.chunks(2) {
        let seed_start = seed_range.get(0).unwrap();
        let seed_length = seed_range.get(1).unwrap();

        let range = *seed_start..seed_start + seed_length;
        println!("Testing range: {:?}", range);

        for seed in range {
            current_lowest_location = current_lowest_location.min(chain_map(seed, &mappings));
        }
    }

    println!("Lowest possible location (part 2): {current_lowest_location}");
}

fn chain_map(seed: u64, mappings: &Vec<Mappings>) -> u64 {
    let mut current_value = seed;

    for mapping in mappings {
        let new_value = mapping.map(current_value);
        // println!("[seed {}] setting currentvalue from {} to {}", seed, current_value, new_value);
        current_value = new_value;
    }

    current_value
}
