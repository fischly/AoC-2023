mod map;

use crate::mapping::map::Map;
use itertools::Itertools;

#[derive(Debug)]
pub struct Mappings {
    pub mappings: Vec<Map>,
}

impl Mappings {
    pub fn map(&self, seed: u64) -> u64 {
        for m in &self.mappings {
            if m.contains_seed(seed) {
                return m.map(seed);
            }
        }

        seed
    }

    pub fn from_string(lines: &[&str]) -> Vec<Mappings> {
        let mut mappings: Vec<Mappings> = Vec::new();
        let mut mapping: Mappings = Mappings {
            mappings: Vec::new(),
        };

        for line in lines {
            // intro to new map
            if line.chars().any(|c| !c.is_digit(10) && c != ' ') {
                if mapping.mappings.len() > 0 {
                    mappings.push(mapping);
                    mapping = Mappings {
                        mappings: Vec::new(),
                    };
                }

                continue;
            }

            // map data
            let (range_dest, range_src, range_len) = line
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            let m = Map {
                dest_range: range_dest..range_dest + range_len,
                source_range: range_src..range_src + range_len,
            };

            mapping.mappings.push(m);
        }

        if mapping.mappings.len() > 0 {
            mappings.push(mapping);
        }

        mappings
    }
}
