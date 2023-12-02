
use std::str::FromStr;

use crate::Color;


#[derive(Debug)]
pub struct Roll {
    pub num: u32,
    pub color: Color,
}

impl Roll {
    pub fn new(input: &str) -> Self {
        // println!("roll new, input = {input:?}");

        let mut iter = input.split(" ");
        let num = iter.next().unwrap().parse::<u32>().unwrap();
        let color = Color::from_str(iter.next().unwrap()).unwrap();

        // println!("roll new, num = {num:?}, color = {color:?}");

        Self { num, color }
    }
}

#[derive(Debug)]
pub struct Rolls {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Rolls {
    pub fn new(input: &str) -> Self {
        // println!("rolls new, input = {input:?}");

        let iter = input.split(",").map(str::trim).map(Roll::new);

        let mut red = Roll {
            color: Color::Red,
            num: 0,
        };
        let mut green = Roll {
            color: Color::Green,
            num: 0,
        };
        let mut blue = Roll {
            color: Color::Blue,
            num: 0,
        };

        for r in iter {
            match r.color {
                Color::Green => green.num = r.num,
                Color::Red => red.num = r.num,
                Color::Blue => blue.num = r.num,
            }
        }

        Self {
            red: red.num,
            green: green.num,
            blue: blue.num,
        }
    }
}