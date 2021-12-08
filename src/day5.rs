use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Line {
    start: (u16, u16),
    end: (u16, u16),
    offset: (i8, i8),
}

impl Line {
    pub fn new(start: (u16, u16), end: (u16, u16)) -> Self {
        debug_assert_ne!(start, end);

        let offset = (
            (end.0 as i32 - start.0 as i32).signum() as i8,
            (end.1 as i32 - start.1 as i32).signum() as i8,
        );

        Self {
            start,
            end: (
                (end.0 as isize + offset.0 as isize) as u16,
                (end.1 as isize + offset.1 as isize) as u16,
            ),
            offset,
        }
    }

    pub fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
}

impl Iterator for Line {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == (0, 0) {
            None
        } else {
            let start = self.start;

            self.start = (
                (start.0 as isize + self.offset.0 as isize) as u16,
                (start.1 as isize + self.offset.1 as isize) as u16,
            );

            if self.start == self.end {
                self.offset = (0, 0);
            }

            Some(start)
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (start, end) = line.split_once(" -> ").ok_or(())?;
        let (start_x, start_y) = start.split_once(',').ok_or(())?;
        let (end_x, end_y) = end.split_once(',').ok_or(())?;

        Ok(Self::new(
            (
                start_x.parse().map_err(|_| ())?,
                start_y.parse().map_err(|_| ())?,
            ),
            (
                end_x.parse().map_err(|_| ())?,
                end_y.parse().map_err(|_| ())?,
            ),
        ))
    }
}

pub fn generator(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn count_overlaps(input: impl AsRef<[Line]>, straight_only: bool) -> usize {
    let mut grid = vec![0; 1000 * 1000];

    input
        .as_ref()
        .iter()
        .cloned()
        .filter(|l| !straight_only || l.is_straight())
        .flatten()
        .for_each(|(x, y)| {
            grid[x as usize + 1000 * y as usize] += 1;
        });

    grid.into_iter().filter(|n| *n > 1).count()
}

pub fn part_1(input: impl AsRef<[Line]>) -> usize {
    count_overlaps(input, true)
}

pub fn part_2(input: impl AsRef<[Line]>) -> usize {
    count_overlaps(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_iter() {
        assert_eq!(
            "9,7 -> 7,7".parse::<Line>().unwrap().collect::<Vec<_>>(),
            vec![(9, 7), (8, 7), (7, 7)]
        );
    }
}
