use std::collections::HashSet;

use crate::Game;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn card(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, map_res(digit1, str::parse))(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, (card, winning_numbers)) = preceded(
        tuple((tag("Card"), space1, digit1, tag(":"), space1)),
        separated_pair(card, tuple((space1, tag("|"), space1)), card),
    )(input)?;

    let winning_numbers = winning_numbers.into_iter().collect::<HashSet<u32>>();
    let game = Game::new(card, winning_numbers);
    Ok((input, game))
}

pub fn parse(input: &str) -> Result<Vec<Game>, nom::Err<nom::error::Error<String>>> {
    let (_, result) =
        all_consuming(separated_list1(line_ending, game))(input).map_err(|err| (err.to_owned()))?;
    Ok(result)
}
