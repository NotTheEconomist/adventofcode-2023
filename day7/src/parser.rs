use std::str::FromStr;

use nom::{
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::{all_consuming, map, map_res},
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::{error::ParseError, Hand};

fn card<CardType: FromStr>(input: &str) -> IResult<&str, CardType> {
    map_res(anychar, |char| char.to_string().parse::<CardType>())(input)
}

fn cards<CardType: FromStr>(input: &str) -> IResult<&str, [CardType; 5]> {
    map_res(count(card, 5), |cards: Vec<CardType>| cards.try_into())(input)
}

fn hand<CardType: FromStr>(input: &str) -> IResult<&str, Hand<CardType>> {
    map(
        separated_pair(cards, space1, map_res(digit1, |d: &str| d.parse::<u32>())),
        |(cards, wager)| Hand::<CardType> { cards, wager },
    )(input)
}

pub fn parse<CardType: FromStr>(input: &'static str) -> Result<Vec<Hand<CardType>>, ParseError> {
    let (_, hands) = all_consuming(separated_list1(line_ending, hand))(input)?;
    Ok(hands)
}
