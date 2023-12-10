#![feature(isqrt)]

mod parser;

pub type Millisecond = i64;
pub type Millimeter = i64;

pub struct Race {
    pub duration: Millisecond,
    pub record: Millimeter,
}

impl Race {
    pub fn new(duration: Millisecond, record: Millimeter) -> Self {
        Self { duration, record }
    }
}

pub fn get_winning_pair(race: &Race) -> (Millisecond, Millisecond) {
    let (r, d) = (race.record, race.duration);
    // quadratic
    let mut high_root = (d + ((i64::pow(d, 2)) - 4 * r).isqrt()) / 2;
    let mut low_root = (d - ((i64::pow(d, 2)) - 4 * r).isqrt()) / 2;
    if high_root * (race.duration - high_root) <= race.record {
        high_root -= 1;
    }
    if low_root * (race.duration - low_root) <= race.record {
        low_root += 1;
    }
    (low_root as Millisecond, high_root as Millisecond)
}

pub fn brute_force_winning_pair(race: &Race) -> (Millisecond, Millisecond) {
    let winners = (1..race.duration)
        .map(|t| (t, (race.duration - t) * t))
        .skip_while(|(_, d)| *d <= race.record)
        .take_while(|(_, d)| *d > race.record)
        .collect::<Vec<_>>();
    let winners = winners.into_iter().map(|(t, _)| t).collect::<Vec<_>>();
    (*winners.first().unwrap(), *winners.last().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn winning_pair() {
        let race = Race {
            duration: 7,
            record: 9,
        };
        let race2 = Race {
            duration: 15,
            record: 40,
        };
        let race3 = Race {
            duration: 30,
            record: 200,
        };
        let (a, b) = get_winning_pair(&race);
        assert_eq!((a, b), (2, 5));
        let (c, d) = brute_force_winning_pair(&race);
        assert_eq!((c, d), (2, 5));
        let (a, b) = get_winning_pair(&race2);
        assert_eq!((a, b), (4, 11));
        let (c, d) = brute_force_winning_pair(&race2);
        assert_eq!((c, d), (4, 11));
        let (a, b) = get_winning_pair(&race3);
        assert_eq!((a, b), (11, 19));
        let (c, d) = brute_force_winning_pair(&race3);
        assert_eq!((c, d), (11, 19));
    }
}

// (D - x) * x > R
// Dx - x**2 > R
// x**2 - Dx + R < 0

// D +/- sqrt((D**2) - 4 * R) / 2)
