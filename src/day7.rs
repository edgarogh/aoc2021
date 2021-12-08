pub fn generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .next()
        .map(|first_line| first_line.split(',').map(|n| n.parse().unwrap()).collect())
        .unwrap_or_default()
}

fn fuel_cost(input: &[u16], attempt: u16) -> usize {
    input
        .into_iter()
        .cloned()
        .map(|n| n.abs_diff(attempt) as usize)
        .sum()
}

pub fn part_1(input: impl AsRef<[u16]>) -> usize {
    let input = input.as_ref();

    (0..1000)
        .map(|i| fuel_cost(input, i))
        .min()
        .unwrap()
}
