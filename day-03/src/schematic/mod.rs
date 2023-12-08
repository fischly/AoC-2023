
mod digitgroup;

use crate::schematic::digitgroup::DigitGroup;

#[derive(Debug)]
pub struct Schematic {
    pub data: Vec<Vec<char>>,
    pub digit_groups: Vec<DigitGroup>,
    pub gears: Vec<(u32, u32)>,
}

impl Schematic {
    pub fn new(input: String) -> Self {
        Self {
            data: Schematic::string_to_2d_vec(&input),
            digit_groups: Schematic::extract_digit_groups(&input),
            gears: Schematic::extract_gears(&input),
        }
    }

    pub fn string_to_2d_vec(input: &String) -> Vec<Vec<char>> {
        let mut v: Vec<Vec<char>> = Vec::new();

        for line in input.split("\n") {
            let mut inner_v: Vec<char> = Vec::new();
            for ch in line.chars() {
                inner_v.push(ch);
            }

            v.push(inner_v);
        }

        v
    }

    pub fn extract_digit_groups(input: &String) -> Vec<DigitGroup> {
        let mut v: Vec<DigitGroup> = Vec::new();
        let mut last_digit: u32 = 0;

        for (i, line) in input.lines().enumerate() {
            let mut chars = line.chars().peekable();
            let mut char_idx: i32 = -1;

            while chars.peek().is_some() {
                let next = chars.next().unwrap();
                char_idx += 1;

                // consume non digits
                if !next.is_digit(10) {
                    if last_digit > 0 {
                        let actual_char_index = if char_idx == 0 { 140 } else { char_idx };
                        let actual_row_index = if char_idx == 0 { i - 1 } else { i };

                        v.push(DigitGroup {
                            digit: last_digit,
                            row: actual_row_index as u32,
                            range: (actual_char_index as u32) - (last_digit.ilog10() + 1)
                                ..=(actual_char_index - 1) as u32,
                        });
                    }

                    last_digit = 0;

                    continue;
                }

                // consume digit
                let next_digit = next.to_digit(10).unwrap();
                last_digit *= 10;
                last_digit += next_digit;
            }
        }

        v
    }

    pub fn extract_gears(input: &String) -> Vec<(u32, u32)> {
        let mut v: Vec<(u32, u32)> = Vec::new();

        for (j, line) in input.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '*' {
                    v.push((i as u32, j as u32));
                }
            }
        }

        v
    }

    pub fn get_element(&self, x: usize, y: usize) -> char {
        *(self.data.get(y).unwrap().get(x).unwrap())
    }

    pub fn neighbours(&self, x: i32, y: i32) -> Vec<char> {
        let mut v: Vec<char> = Vec::new();

        for j in y - 1..=y + 1 {
            for i in x - 1..=x + 1 {
                if i == x && j == y {
                    continue;
                }
                if i < 0 || j < 0 || i > 139 || j > 139 {
                    continue;
                }

                v.push(*(&self.get_element(i as usize, j as usize)));
            }
        }

        v
    }

    pub fn contains_symbol(&self, input: Vec<char>) -> bool {
        for c in "*%/-$&+#=@".chars() {
            if input.contains(&c) {
                return true;
            }
        }
        false
    }

    pub fn symbol_in_neighbourhood(&self, x: i32, y: i32) -> bool {
        self.contains_symbol(self.neighbours(x, y))
    }
}