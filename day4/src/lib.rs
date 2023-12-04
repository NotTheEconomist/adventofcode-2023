use std::collections::HashSet;

pub mod parser;

#[derive(Debug, Clone)]
pub struct Game {
    card: Vec<u32>,
    winning_numbers: HashSet<u32>,
}

impl Game {
    pub fn new(card: Vec<u32>, winning_numbers: HashSet<u32>) -> Self {
        Self {
            card,
            winning_numbers,
        }
    }

    pub fn score(&self) -> u32 {
        let count = self
            .card
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count();

        match count {
            0 => 0,
            n => 2u32.pow(n as u32 - 1),
        }
    }

    pub fn winning_number_count(&self) -> usize {
        self.card
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count()
    }
}
