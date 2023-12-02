use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        let col = match input.to_lowercase().trim() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        };
        return col;
    }
}