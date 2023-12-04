use itertools::Itertools;

mod util;

fn get_card_rewards(input: &str) -> u32 {
    let (winning, own) = input
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split("|")
        .map(str::trim)
        .map(|s| {
            s.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect_tuple()
        .unwrap();

    winning
        .iter()
        .map(|w| if own.contains(w) { 1 } else { 0 })
        .sum::<u32>()
}

fn main() {
    let input = util::read_input();
    let cards = input.lines().map(get_card_rewards).collect::<Vec<_>>();

    // part 1
    let point_sum: u32 = cards
        .iter()
        .map(|w| if w > &0 { 2u32.pow(w - 1) } else { 0 })
        .sum();

    println!("Sum of points (part 1): {:?}", point_sum);

    // part 2
    let mut card_stack: Vec<u32> = (1..=cards.len() as u32).rev().collect();
    let mut counter = 0;

    while !card_stack.is_empty() {
        let card = card_stack.pop().unwrap();
        let rew = cards.get((card - 1) as usize).unwrap();

        card_stack.extend(card + 1..card + 1 + rew);

        counter += 1;
    }

    println!("Collected cards (part 2): {:?}", counter);
}
