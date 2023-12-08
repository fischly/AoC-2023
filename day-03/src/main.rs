mod schematic;
mod util;

use crate::schematic::Schematic;

fn main() {
    let input = util::read_input();

    let s = Schematic::new(input.clone());

    // ===== Part 1 ===== //

    let mut digit_sum = 0;
    for dg in s.digit_groups.to_vec() {
        for x in dg.range.clone() {
            if s.symbol_in_neighbourhood(x as i32, dg.row as i32) {
                digit_sum += dg.digit;
                break;
            }
        }
    }

    println!("Part sum (ex1): {:?}", digit_sum);

    // ===== Part 2 ===== //

    let mut gear_factor_sum = 0;

    for g in s.gears.clone() {
        let parts = s
            .digit_groups
            .iter()
            .filter(|dg| {
                // digit group is in a wrong row
                if dg.row < g.1 - 1 || dg.row > g.1 + 1 {
                    false
                } else {
                    let start = *dg.range.start() as i32;
                    let end = *dg.range.end() as i32;

                    ((start - 1)..(end + 2)).contains(&(g.0 as i32))
                }
            })
            .collect::<Vec<_>>();

        if parts.len() == 2 {
            let gear_factor = parts.get(0).unwrap().digit * parts.get(1).unwrap().digit;
            gear_factor_sum += gear_factor;
        }
    }

    print!("Gear factor ratio (part 2): {}", gear_factor_sum);
}
