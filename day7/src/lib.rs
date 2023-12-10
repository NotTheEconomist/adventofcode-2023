pub mod error;
pub mod parser;

use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Two => "2",
                Self::Three => "3",
                Self::Four => "4",
                Self::Five => "5",
                Self::Six => "6",
                Self::Seven => "7",
                Self::Eight => "8",
                Self::Nine => "9",
                Self::Ten => "T",
                Self::Jack => "J",
                Self::Queen => "Q",
                Self::King => "K",
                Self::Ace => "A",
            }
        )
    }
}

impl FromStr for Card {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            other => Err(error::ParseError::CardFromStr {
                input: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CardWithJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
impl From<CardWithJoker> for Card {
    fn from(value: CardWithJoker) -> Card {
        format!("{}", value).parse().unwrap()
    }
}
impl From<Card> for CardWithJoker {
    fn from(value: Card) -> Self {
        format!("{}", value).parse().unwrap()
    }
}
impl Display for CardWithJoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Joker => "J",
                Self::Two => "2",
                Self::Three => "3",
                Self::Four => "4",
                Self::Five => "5",
                Self::Six => "6",
                Self::Seven => "7",
                Self::Eight => "8",
                Self::Nine => "9",
                Self::Ten => "T",
                Self::Queen => "Q",
                Self::King => "K",
                Self::Ace => "A",
            }
        )
    }
}
impl FromStr for CardWithJoker {
    type Err = error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Joker),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            other => Err(error::ParseError::CardFromStr {
                input: other.to_string(),
            }),
        }
    }
}

pub trait HasHandValue {
    fn hand_strength(&self) -> HandValue;
}

#[derive(Debug)]
pub struct Hand<CardType> {
    pub cards: [CardType; 5],
    pub wager: u32,
}
impl HasHandValue for Hand<Card> {
    fn hand_strength(&self) -> HandValue {
        let counts = self
            .cards
            .iter()
            .fold(HashMap::<Card, u8>::new(), |mut acc, card| {
                acc.entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                acc
            });
        counts
            .values()
            .fold(HandValue::HighCard, |acc, count| match (acc, count) {
                (acc, 1) => acc,
                (HandValue::HighCard, 2) => HandValue::Pair,
                (HandValue::Pair, 2) => HandValue::TwoPair,
                (HandValue::ThreeOfAKind, 2) => HandValue::FullHouse,
                (HandValue::HighCard, 3) => HandValue::ThreeOfAKind,
                (HandValue::Pair, 3) => HandValue::FullHouse,
                (_, 4) => HandValue::FourOfAKind,
                (_, 5) => HandValue::FiveOfAKind,
                (acc, _) => acc,
            })
    }
}
impl HasHandValue for Hand<CardWithJoker> {
    fn hand_strength(&self) -> HandValue {
        let counts =
            self.cards
                .iter()
                .fold(HashMap::<CardWithJoker, u8>::new(), |mut acc, card| {
                    acc.entry(*card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    acc
                });
        let num_jokers = counts.get(&CardWithJoker::Joker).copied().unwrap_or(0u8);
        let value_before_jokers = counts
            .iter()
            .filter_map(|(card, count)| {
                if matches!(*card, CardWithJoker::Joker) {
                    None
                } else {
                    Some(count)
                }
            })
            .fold(HandValue::HighCard, |acc, count| match (acc, count) {
                (acc, 1) => acc,
                (HandValue::HighCard, 2) => HandValue::Pair,
                (HandValue::Pair, 2) => HandValue::TwoPair,
                (HandValue::ThreeOfAKind, 2) => HandValue::FullHouse,
                (HandValue::HighCard, 3) => HandValue::ThreeOfAKind,
                (HandValue::Pair, 3) => HandValue::FullHouse,
                (_, 4) => HandValue::FourOfAKind,
                (_, 5) => HandValue::FiveOfAKind,
                (acc, _) => acc,
            });
        match (value_before_jokers, num_jokers) {
            (value, 0) => value,
            (HandValue::HighCard, 1) => HandValue::Pair,
            (HandValue::HighCard, 2) => HandValue::ThreeOfAKind,
            (HandValue::HighCard, 3) => HandValue::FourOfAKind,
            (HandValue::HighCard, 4) => HandValue::FiveOfAKind,
            (HandValue::HighCard, 5) => HandValue::FiveOfAKind, // This special case is only possible when the hand is JJJJJ
            (HandValue::Pair, 1) => HandValue::ThreeOfAKind,
            (HandValue::Pair, 2) => HandValue::FourOfAKind,
            (HandValue::Pair, 3) => HandValue::FiveOfAKind,
            (HandValue::TwoPair, 1) => HandValue::FullHouse,
            (HandValue::ThreeOfAKind, 1) => HandValue::FourOfAKind,
            (HandValue::ThreeOfAKind, 2) => HandValue::FiveOfAKind,
            (HandValue::FourOfAKind, 1) => HandValue::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandValue {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
