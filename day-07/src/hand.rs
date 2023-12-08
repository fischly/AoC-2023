use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn has_five_of_a_kind(occs: &HashMap<char, i32>) -> bool {
        occs.keys().len() == 1
    }

    pub fn has_four_of_a_kind(occs: &HashMap<char, i32>) -> bool {
        occs.values().contains(&4)
    }

    pub fn has_full_house(occs: &HashMap<char, i32>) -> bool {
        occs.values().contains(&3) && occs.values().contains(&2)
    }

    pub fn has_three_of_a_kind(occs: &HashMap<char, i32>) -> bool {
        match occs.values().find(|p| p == &&3) {
            Some(_) => occs.values().filter(|p| p == &&1).count() == 2,
            None => false,
        }
    }

    pub fn has_two_pair(occs: &HashMap<char, i32>) -> bool {
        occs.values().filter(|p| p == &&2).count() == 2
    }

    pub fn has_one_pair(occs: &HashMap<char, i32>) -> bool {
        occs.values().filter(|p| p == &&2).count() == 1 && !occs.values().contains(&3)
    }

    pub fn get_highest_type(hand: &String) -> HandType {
        let occurences = hand.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        if HandType::has_five_of_a_kind(&occurences) {
            return HandType::FiveOfAKind;
        } else if HandType::has_four_of_a_kind(&occurences) {
            return HandType::FourOfAKind;
        } else if HandType::has_full_house(&occurences) {
            return HandType::FullHouse;
        } else if HandType::has_three_of_a_kind(&occurences) {
            return HandType::ThreeOfAKind;
        } else if HandType::has_two_pair(&occurences) {
            return HandType::TwoPair;
        } else if HandType::has_one_pair(&occurences) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: String,
    pub original_card: Option<String>,
    pub bid: u32,
}

fn char_comp(ch1: char, ch2: char, use_joker: Option<bool>) -> std::cmp::Ordering {
    let char_values = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', if use_joker.unwrap_or(false) { 1 } else { 11 }),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let val_1 = char_values.get(&ch1).unwrap();
    let val_2 = char_values.get(&ch2).unwrap();

    if val_1 > val_2 {
        std::cmp::Ordering::Greater
    } else if val_1 < val_2 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

pub trait HandTypeComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
}
pub trait CharComperable {
    fn cmp(&self, other: &Self, use_joker: Option<bool>) -> std::cmp::Ordering;
}
pub trait CharHandTypeComperable {
    fn cmp(&self, other: &Self, use_joker: Option<bool>) -> std::cmp::Ordering;
}

impl HandTypeComparable for String {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let own_ht = HandType::get_highest_type(self) as usize;
        let other_ht = HandType::get_highest_type(other) as usize;

        match own_ht {
            own if own > other_ht => std::cmp::Ordering::Greater,
            own if own < other_ht => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl CharComperable for String {
    fn cmp(&self, other: &Self, use_joker: Option<bool>) -> std::cmp::Ordering {
        for (char_self, char_other) in self.chars().zip(other.chars()) {
            let char_comparison = char_comp(char_self, char_other, use_joker);

            if char_comparison == std::cmp::Ordering::Equal {
                continue;
            }

            return char_comparison;
        }

        std::cmp::Ordering::Equal
    }
}

impl CharHandTypeComperable for String {
    fn cmp(&self, other: &Self, use_joker: Option<bool>) -> std::cmp::Ordering {
        match HandTypeComparable::cmp(self, other) {
            std::cmp::Ordering::Equal => CharComperable::cmp(self, other, use_joker),
            res => res,
        }
    }
}
