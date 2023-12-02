use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{Cube, Game};

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, result) = separated_pair(
        map_res(digit1, |m: &str| m.parse::<u32>()),
        char(' '),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;
    let result = match result {
        (count, "red") => Cube::Red(count),
        (count, "green") => Cube::Green(count),
        (count, "blue") => Cube::Blue(count),
        (_, &_) => unreachable!("color can only be red, green, blue"),
    };
    Ok((input, result))
}

fn pull(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), cube)(input)
}

fn pulls(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    separated_list1(tag("; "), pull)(input)
}

fn game_id(input: &str) -> IResult<&str, u32> {
    preceded(tag("Game "), map_res(digit1, |m: &str| m.parse::<u32>()))(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, (id, pulls)) = separated_pair(game_id, tag(": "), pulls)(input)?;
    let game = Game {
        id,
        bag_state: pulls.into(),
    };
    Ok((input, game))
}

pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

#[cfg(test)]
mod test {
    use nom::combinator::all_consuming;

    use super::*;
    use crate::BagState;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn parse_game() {
        let given = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            bag_state: BagState::new(4, 2, 6),
        };
        let (_, result) = all_consuming(game)(given).expect("Must parse");
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_games() {
        let given = INPUT;
        let expected = vec![
            Game {
                id: 1,
                bag_state: BagState::new(4, 2, 6),
            },
            Game {
                id: 2,
                bag_state: BagState::new(1, 3, 4),
            },
            Game {
                id: 3,
                bag_state: BagState::new(20, 13, 6),
            },
            Game {
                id: 4,
                bag_state: BagState::new(14, 3, 15),
            },
            Game {
                id: 5,
                bag_state: BagState::new(6, 3, 2),
            },
        ];
        let (_, result) = all_consuming(games)(given).expect("Must parse");
        assert_eq!(result, expected);
    }
}
