mod util;
mod hand;

use itertools::Itertools;
use crate::hand::{ CharComperable, CharHandTypeComperable, HandTypeComparable, Hand };


fn replace_js(input: String, current_best: String) -> String {
    if input.contains("J") {
        let mut best = current_best.clone();
        for new_char in "AKQT98765432".chars() {
            best = replace_js(
                input.replacen("J", new_char.to_string().as_str(), 1),
                best.clone(),
            );
        }

        best
    } else {
        if HandTypeComparable::cmp(&input, &current_best) == std::cmp::Ordering::Greater {
            input
        } else {
            current_best
        }
    }
}

fn main() {
    let input = util::read_input();
    let cards = input
        .lines()
        .map(|l| l.split(" "))
        .map(|mut m| Hand {
            cards: m.next().unwrap().to_string(),
            bid: m.next().unwrap().parse::<u32>().unwrap(),
            original_card: None,
        })
        .collect::<Vec<_>>();

    // part 1

    // sort cards
    let sorted = cards
        .iter()
        .sorted_by(|a, b| CharHandTypeComperable::cmp(&a.cards, &b.cards, Some(false)))
        .collect::<Vec<_>>();

    // calculate winnings
    let mut sum: u64 = 0;
    for (i, h) in sorted.iter().enumerate() {
        sum += (h.bid as u64) * ((i as u64) + 1);
    }

    println!("sum (part 1): {}", sum);

    // part 2

    // find best cards by replacing Js
    let new_cards = cards
        .iter()
        .map(|c| Hand {
            bid: c.bid,
            cards: replace_js(c.cards.clone(), c.cards.clone()),
            original_card: Some(c.cards.clone()),
        })
        .collect::<Vec<_>>();

    // sort cards
    let sorted = new_cards
        .iter()
        .sorted_by(|a, b| {
            let type_comp = HandTypeComparable::cmp(&a.cards, &b.cards);

            if type_comp != std::cmp::Ordering::Equal {
                type_comp
            } else {
                CharComperable::cmp(
                    a.original_card.as_ref().unwrap(),
                    &b.original_card.as_ref().unwrap(),
                    Some(true),
                )
            }
        })
        .collect::<Vec<_>>();

    // calculate winnings
    let mut sum: u64 = 0;
    for (i, h) in sorted.iter().enumerate() {
        sum += (h.bid as u64) * ((i as u64) + 1);
    }

    println!("sum (part 2): {}", sum);
}
