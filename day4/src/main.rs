use day4::parser;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: &str) -> anyhow::Result<u32> {
    let games = parser::parse(input)?;
    Ok(games.into_iter().fold(0, |acc, game| acc + game.score()))
}

fn solve_part2(input: &str) -> anyhow::Result<u32> {
    let original_games = parser::parse(input)?;
    let mut cards_held = vec![1; original_games.len()];
    for (idx, game) in original_games.into_iter().enumerate() {
        let wins = game.winning_number_count();
        if wins > 0 {
            let count = cards_held[idx];
            for winning_index in idx + 1..idx + wins + 1 {
                cards_held[winning_index] += count;
            }
        }
    }
    Ok(cards_held.into_iter().sum())
}

fn main() -> anyhow::Result<()> {
    let part1 = solve_part1(INPUT)?;
    println!("part1: {}", part1);
    let part2 = solve_part2(INPUT)?;
    println!("part2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solve_part1() {
        let solution = solve_part1(INPUT).expect("must parse");
        assert_eq!(solution, 13);
    }
    #[test]
    fn test_solve_part2() {
        let solution = solve_part2(INPUT).expect("must parse");
        assert_eq!(solution, 30);
    }
}
