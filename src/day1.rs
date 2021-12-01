pub fn generator<'a>(input: &'a str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: impl AsRef<[usize]>) -> usize {
    input
        .as_ref()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

pub fn part_2(input: impl AsRef<[usize]>) -> usize {
    let sum = input
        .as_ref()
        .windows(3)
        .map(|s| s.iter().cloned().sum())
        .collect::<Vec<_>>();

    part_1(sum)
}
