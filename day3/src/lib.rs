#![feature(impl_trait_in_assoc_type)]

use itertools::Itertools;
use std::collections::HashSet;

trait Adjacence<T>
where
    T: PartialEq + Eq,
{
    fn is_adjacent_to<O: Adjacence<T>>(&self, other: &O) -> bool;
    fn adjacencies(&self) -> impl Iterator<Item = T>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BoundingBox {
    topleft: (usize, usize),
    bottomright: (usize, usize),
}

impl Adjacence<(usize, usize)> for BoundingBox {
    fn is_adjacent_to<O: Adjacence<(usize, usize)>>(&self, other: &O) -> bool {
        let ((x1, y1), (x2, y2)) = (self.topleft, self.bottomright);
        let bounds = (x1..=x2).zip(y1..=y2).collect::<HashSet<_>>();
        other.adjacencies().any(|pos| bounds.contains(&pos))
    }

    fn adjacencies(&self) -> impl Iterator<Item = (usize, usize)> {
        let (x1, x2) = (self.topleft.0, self.bottomright.0);
        let (y1, y2) = (self.topleft.1, self.bottomright.1);
        (x1 - 1..=x2 + 1)
            .map(move |x| (x - 1, y1)) // top side
            .chain(
                (y1 - 1..=y2 + 1).map(move |y| (x2 + 1, y)), // right side
            )
            .chain(
                (x1 - 1..=x2 + 1).map(move |x| (x, y2 - 1)), // bottom side
            )
            .chain(
                (y1 - 1..=y2 + 1).map(move |y| (x1 - 1, y)), // Left side
            )
            .unique()
    }
}

impl Adjacence<(usize, usize)> for (usize, usize) {
    fn is_adjacent_to<O: Adjacence<(usize, usize)>>(&self, other: &O) -> bool {
        other.adjacencies().any(|pos| pos == *self)
    }

    fn adjacencies(&self) -> impl Iterator<Item = (usize, usize)> {
        coords::adjacencies_to(*self).into_iter()
    }
}
mod coords {
    pub fn west_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_sub(1)?, y))
    }
    pub fn northwest_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_sub(1)?, y.checked_sub(1)?))
    }
    pub fn north_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x, y.checked_sub(1)?))
    }
    pub fn northeast_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_add(1)?, y.checked_sub(1)?))
    }
    pub fn east_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_add(1)?, y))
    }
    pub fn southeast_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_add(1)?, y.checked_add(1)?))
    }
    pub fn south_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x, y.checked_add(1)?))
    }
    pub fn southwest_from((x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some((x.checked_sub(1)?, y.checked_add(1)?))
    }

    pub fn adjacencies_to((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        vec![
            northwest_from((x, y)),
            north_from((x, y)),
            northeast_from((x, y)),
            east_from((x, y)),
            southeast_from((x, y)),
            south_from((x, y)),
            southwest_from((x, y)),
            west_from((x, y)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn get_value_on_grid<V>(grid: &[Vec<V>], (coord_x, coord_y): (usize, usize)) -> Option<&V> {
        grid.get(coord_y)?.get(coord_x)
    }

    pub fn get_optional_value_on_grid<V>(
        grid: &[Vec<V>],
        coord: Option<(usize, usize)>,
    ) -> Option<&V> {
        get_value_on_grid(grid, coord?)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Part {
    id: u32,
    location: BoundingBox,
}
impl Part {
    pub fn from_coordinate_on_grid(grid: &[Vec<char>], coordinate: (usize, usize)) -> Option<Self> {
        let mut final_number_acc = String::new();
        let val = *coords::get_value_on_grid(grid, coordinate)?;
        let mut topleft = coordinate;
        let mut bottomright = coordinate;
        if val.is_numeric() {
            final_number_acc.push(val);
            // Add from the left
            let mut cur_pos = coordinate;
            while let Some(left) =
                coords::get_optional_value_on_grid(grid, coords::west_from(cur_pos))
            {
                if left.is_numeric() {
                    final_number_acc.insert(0, *left);
                    cur_pos = coords::west_from(cur_pos)?;
                    topleft = cur_pos;
                } else {
                    break;
                }
            }
            // Add from the right
            cur_pos = coordinate;
            while let Some(right) =
                coords::get_optional_value_on_grid(grid, coords::east_from(cur_pos))
            {
                if right.is_numeric() {
                    final_number_acc.push(*right);
                    cur_pos = coords::east_from(cur_pos)?;
                    bottomright = cur_pos;
                } else {
                    break;
                }
            }
        }

        if final_number_acc.is_empty() {
            None
        } else {
            let id = final_number_acc
                .parse::<u32>()
                .expect("All numerics should be okay to parse");
            let location = BoundingBox {
                topleft,
                bottomright,
            };
            Some(Part { id, location })
        }
    }
}

pub struct Schematic {
    grid: Vec<Vec<char>>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Gear {
    adjacent_parts: [Part; 2],
}

impl Gear {
    pub fn gear_ratio(&self) -> u64 {
        self.adjacent_parts[0].id as u64 * self.adjacent_parts[1].id as u64
    }
}

impl Schematic {
    pub fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        // Gather the non-empty, non-numeric spaces
        let symbols = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, val)| match val {
                        '.' => None,
                        val if val.is_numeric() => None,
                        val => Some(((x, y), val)),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let parts = symbols
            .into_iter()
            .flat_map(|(pos, _)| {
                pos.adjacencies()
                    .filter_map(|adj| Part::from_coordinate_on_grid(&grid, adj))
                    .collect::<Vec<_>>()
            })
            .unique()
            .collect::<Vec<_>>();

        Self { grid, parts }
    }

    pub fn find_gears(&self) -> Vec<Gear> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, val)| match val {
                        '*' => Some(((x, y), val)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .filter_map(|(coord, _)| {
                let adjacent_parts = coord
                    .adjacencies()
                    .filter_map(|adj| Part::from_coordinate_on_grid(&self.grid, adj))
                    .unique()
                    .collect::<Vec<_>>();

                if adjacent_parts.len() == 2 {
                    Some(Gear {
                        adjacent_parts: [adjacent_parts[0], adjacent_parts[1]],
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn sum(&self) -> u32 {
        self.parts.iter().map(|part| part.id).sum::<u32>()
    }

    pub fn parts(&self) {
        println!(
            "parts: {}",
            self.parts.iter().map(|part| part.id.to_string()).join(", "),
        )
    }

    // fn get_adjacencies(&self, (x, y): (usize, usize)) -> Vec<char> {
    //     coords::adjacencies_to((x, y))
    //         .into_iter()
    //         .filter_map(|coordinates| coords::get_value_on_grid(&self.grid, coordinates).copied())
    //         .collect::<Vec<_>>()
    // }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_schematic() {
        let data = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let schematic = Schematic::new(data);
        let expected_parts = vec![467, 35, 633, 617, 592, 664, 755, 598];
        assert_eq!(
            schematic
                .parts
                .into_iter()
                .map(|part| part.id)
                .collect::<Vec<_>>(),
            expected_parts
        );
    }
}
