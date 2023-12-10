use std::str::FromStr;

use day7::{parser, Card, CardWithJoker, Hand, HasHandValue};

const INPUT: &str = include_str!("input.txt");

fn solve<CardType: FromStr + Ord + Copy>(input: &'static str) -> u32
where
    Hand<CardType>: HasHandValue,
{
    let mut hands: Vec<Hand<CardType>> = parser::parse(input).expect("hands must parse");
    hands.sort_by_key(|hand| (hand.hand_strength(), hand.cards));
    (1u32..)
        .zip(hands.into_iter())
        .map(|(rank, Hand { cards: _, wager })| wager * rank)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let part1 = solve::<Card>(INPUT);
    println!("{}", part1);
    let part2 = solve::<CardWithJoker>(INPUT);
    println!("{}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_solve_part1() {
        let result = solve::<Card>(INPUT);
        assert_eq!(result, 6440);
    }
}
