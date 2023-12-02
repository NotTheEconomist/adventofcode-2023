use day2::parser;
use day2::BagState;
use day2::Game;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(games: Vec<Game>) -> u32 {
    let given = BagState::new(12, 13, 14);
    (1u32..)
        .zip(games)
        .filter_map(|(idx, game)| {
            if game.bag_state.possible_with_given_state(&given) {
                Some(idx as u32)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(games: Vec<Game>) -> u64 {
    games
        .into_iter()
        .map(|game| game.bag_state.power() as u64)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let (_, games) = parser::games(INPUT).expect("Must parse");
    let part1 = solve_part1(games.clone());
    let part2 = solve_part2(games);
    println!("part1: {}\npart2: {}", part1, part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_solve_part1() {
        let (_, games) = parser::games(INPUT).expect("Must parse");
        assert_eq!(solve_part1(games), 8);
    }

    #[test]
    fn test_solve_part2() {
        let (_, games) = parser::games(INPUT).expect("Must parse");
        assert_eq!(solve_part2(games), 2286);
    }
}
