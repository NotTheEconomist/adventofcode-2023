use day3::Schematic;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    schematic.sum()
}

fn solve_part2(input: &str) -> u64 {
    let schematic = Schematic::new(input);
    schematic
        .find_gears()
        .into_iter()
        .map(|gear| gear.gear_ratio())
        .sum()
}

fn main() -> anyhow::Result<()> {
    let part1 = solve_part1(INPUT);
    println!("part1: {}", part1);
    let part2 = solve_part2(INPUT);
    println!("part2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_solve_part1() {
        let solution = solve_part1(INPUT);
        assert_eq!(solution, 4361);
    }
    #[test]
    fn test_solve_part2() {
        let solution = solve_part2(INPUT);
        assert_eq!(solution, 467835);
    }
}
