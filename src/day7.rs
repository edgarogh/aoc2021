pub fn generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .next()
        .map(|first_line| first_line.split(',').map(|n| n.parse().unwrap()).collect())
        .unwrap_or_default()
}

fn fuel_cost_1(input: &[u16], attempt: u16) -> usize {
    input
        .into_iter()
        .cloned()
        .map(|n| n.abs_diff(attempt) as usize)
        .sum()
}

fn fuel_cost_2(input: &[u16], attempt: u16) -> usize {
    input
        .into_iter()
        .cloned()
        .map(|n| n.abs_diff(attempt) as usize)
        .map(|n| n * (n + 1) / 2)
        .sum()
}

fn best_attempt(input: impl AsRef<[u16]>, fuel_cost: impl Fn(&[u16], u16) -> usize) -> usize {
    let input = input.as_ref();

    (0..1000)
        .map(|i| fuel_cost(input, i))
        .min()
        .unwrap()
}

pub fn part_1(input: impl AsRef<[u16]>) -> usize {
    best_attempt(input, fuel_cost_1)
}

pub fn part_2(input: impl AsRef<[u16]>) -> usize {
    best_attempt(input, fuel_cost_2)
}
