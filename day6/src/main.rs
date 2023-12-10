use day6::{get_winning_pair, Race};

fn main() -> anyhow::Result<()> {
    let races = vec![
        Race::new(49, 298),
        Race::new(78, 1185),
        Race::new(79, 1066),
        Race::new(80, 1181),
    ];

    let counts: i64 = races
        .iter()
        .map(get_winning_pair)
        .map(|(start, end)| end - start + 1)
        .product();
    println!("{}", counts);

    let race = Race::new(49787980, 298118510661181);
    let (a, b) = get_winning_pair(&race);
    println!("{}", b - a + 1);
    Ok(())
}
