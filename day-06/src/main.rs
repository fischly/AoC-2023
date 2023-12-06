mod util;

use itertools::Itertools;

fn main() {
    let input = util::read_input();

    // parse input
    let (times, distances) = input
        .lines()
        .map(|r| {
            r.split(":")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|c| !c.is_empty())
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        // .map(|e| e.map(str::trim).collect::<Vec<_>>())
        .collect_tuple()
        .unwrap();

    // part 1

    let mut counter: u32 = 0;
    let mut total: u32 = 1;

    for (t, d) in std::iter::zip(&times, &distances) {
        for time_pressed in 0..=*t {
            let acc = time_pressed;
            let time_left = t - time_pressed;
            let traveled_dist = acc * time_left;

            if traveled_dist > *d {
                counter += 1;
            }
        }

        total *= counter;
        counter = 0;
    }

    println!("Total (part 1): {}", total);

    // part 2

    let time = times.iter().join("").parse::<u64>().unwrap();
    let dist = distances.iter().join("").parse::<u64>().unwrap();

    let mut counter: u64 = 0;

    for time_pressed in 0..time {
        let acc = time_pressed;
        let time_left = time - time_pressed;
        let traveled_dist = acc * time_left;

        if traveled_dist > dist {
            counter += 1;
        }
    }

    println!("Total (part 2): {}", counter);
}
