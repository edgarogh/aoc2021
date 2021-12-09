const fn count_bits(n: u8) -> u8 {
    const NIBBLES: &[u8; 16] = &[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

    let n = n as usize;
    NIBBLES[n & 0xf] + NIBBLES[n >> 4]
}

pub fn parse_digit(digit: &str) -> u8 {
    use std::simd::*;

    let digit = digit.as_bytes();
    debug_assert!(digit.len() <= 7);
    let chars = Simd::gather_or(
        digit,
        Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7]),
        Simd::splat(b'h'),
    );

    let shifted = Simd::splat(1) << (chars - Simd::splat(b'a'));
    shifted.horizontal_or() & 0b1111111
}

pub fn generator(input: &str) -> Vec<[Vec<u8>; 2]> {
    input
        .lines()
        .map(|line| {
            let (mut input, mut output) = (Vec::new(), Vec::new());
            let mut current_vec = &mut input;

            for word in line.split_ascii_whitespace() {
                if word == "|" {
                    current_vec = &mut output;
                } else {
                    current_vec.push(parse_digit(word));
                }
            }

            [input, output]
        })
        .collect()
}

pub fn part_1(input: impl AsRef<[[Vec<u8>; 2]]>) -> usize {
    input
        .as_ref()
        .into_iter()
        .flat_map(|[_, v]| v)
        .filter(|n| [2, 4, 3, 7].contains(&count_bits(**n)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit("abc"), 0b0000111);
        assert_eq!(parse_digit("facbd"), 0b0101111);
        assert_eq!(parse_digit("gae"), 0b1010001);
    }
}
