

pub mod game;
pub mod util;

use game::Game;
use game::color::Color;
use game::rolls::Rolls;
use crate::game::rolls;

fn main() {
    let input_str = util::read_input();
    let input = input_str.as_str();

    let games = input.lines().map(Game::new);

    // ===== PART 1 =====

    let possible_games = games.clone()
        .filter(|game: &Game| {
            game.rolls
                .iter()
                .all(|rolls: &rolls::Rolls| rolls.red <= 12 && rolls.green <= 13 && rolls.blue <= 14)
            // .any(|rolls: &Rolls| rolls.red <= 12 && rolls.green <= 13 && rolls.blue <= 14)
        })
        .map(|game| game.id);
    let id_sum = possible_games.reduce(|acc, ele| acc + ele).unwrap();

    println!("ID sum of possible games (part 1): {id_sum}");


    // ===== PART 2 =====

    let minimum_cubes = games.clone().map(|game| {
        let red_max = game.rolls.iter().map(|r| r.red).max().unwrap();
        let green_max = game.rolls.iter().map(|r| r.green).max().unwrap();
        let blue_max = game.rolls.iter().map(|r| r.blue).max().unwrap();
        
        (red_max, green_max, blue_max)
    });

    let power_set_sum: u32 = minimum_cubes.map(|(red, green, blue)| red * green * blue).sum();

    println!("Power set sum: {power_set_sum}");
}

