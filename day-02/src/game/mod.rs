

pub mod color;
pub mod rolls;

use crate::Rolls;



#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub rolls: Vec<Rolls>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let mut split = input.split(":").map(str::trim);
        let id = split
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        let rolls_str = split.next().unwrap();
        let rolls = rolls_str
            .split("; ")
            .map(str::trim)
            .map(Rolls::new)
            .collect();

        Self { rolls, id }
    }
}