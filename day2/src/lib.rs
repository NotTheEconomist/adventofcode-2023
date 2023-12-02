pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BagState {
    red: u32,
    green: u32,
    blue: u32,
}

impl BagState {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn add_state(mut self, other: BagState) -> Self {
        if other.red > self.red {
            self.red = other.red
        }
        if other.green > self.green {
            self.green = other.green
        }
        if other.blue > self.blue {
            self.blue = other.blue
        }
        self
    }

    pub fn possible_with_given(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
    pub fn possible_with_given_state(&self, other: &BagState) -> bool {
        self.possible_with_given(other.red, other.green, other.blue)
    }
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
impl Default for BagState {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl From<Vec<Cube>> for BagState {
    fn from(val: Vec<Cube>) -> Self {
        let (red, green, blue) = val.into_iter().fold(
            (0u32, 0u32, 0u32),
            |(red, green, blue), cube: Cube| match cube {
                Cube::Red(count) => (red + count, green, blue),
                Cube::Green(count) => (red, green + count, blue),
                Cube::Blue(count) => (red, green, blue + count),
            },
        );
        Self::new(red, green, blue)
    }
}

impl From<Vec<Vec<Cube>>> for BagState {
    fn from(value: Vec<Vec<Cube>>) -> Self {
        value
            .into_iter()
            .map(BagState::from)
            .fold(Self::default(), |acc, bagstate| acc.add_state(bagstate))
    }
}

pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub bag_state: BagState,
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
    fn test_is_possible() {
        let game = Game {
            id: 1,
            bag_state: BagState::new(4, 2, 6),
        };
        assert!(game.bag_state.possible_with_given(12, 13, 14));

        let game = Game {
            id: 1,
            bag_state: BagState::new(17, 2, 6),
        }; // Too many reds!

        assert!(!game.bag_state.possible_with_given(12, 13, 14));
    }
}
