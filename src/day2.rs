#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward = 9,
    Down = 6,
    Up = 4,
}

impl Direction {
    pub fn from_len(len: usize) -> Self {
        match len {
            9 => Self::Forward,
            6 => Self::Down,
            4 => Self::Up,
            _ => unreachable!(),
        }
    }
}

pub fn generator(input: &str) -> Vec<(Direction, u8)> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|l| {
            (
                Direction::from_len(l.len()),
                l.iter().last().unwrap() - 0x30,
            )
        })
        .collect()
}

pub fn part_1(input: impl AsRef<[(Direction, u8)]>) -> usize {
    let pos = input
        .as_ref()
        .iter()
        .cloned()
        .map(|(x, b)| (x, b as usize))
        .fold(
            (0usize, 0usize),
            |(x, y), (direction, len)| match direction {
                Direction::Forward => (x + len, y),
                Direction::Down => (x, y + len),
                Direction::Up => (x, y - len),
            },
        );

    pos.0 * pos.1
}

pub fn part_2(input: impl AsRef<[(Direction, u8)]>) -> usize {
    let pos = input
        .as_ref()
        .iter()
        .cloned()
        .map(|(x, b)| (x, b as usize))
        .fold(
            (0usize, 0usize, 0usize),
            |(x, y, aim), (direction, len)| match direction {
                Direction::Forward => (x + len, y + len * aim, aim),
                Direction::Down => (x, y, aim + len),
                Direction::Up => (x, y, aim - len),
            },
        );

    pos.0 * pos.1
}
