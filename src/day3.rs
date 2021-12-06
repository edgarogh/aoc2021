use std::simd::{i8x16, mask8x16, u16x16, Mask, Simd};

fn mask8x16_to_u16(mask: mask8x16) -> u16 {
    let mask_shifted = Mask::from(mask).select(u16x16::splat(1), u16x16::splat(0))
        << Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    mask_shifted.horizontal_sum()
}

pub fn generator(input: &str) -> Vec<mask8x16> {
    input
        .lines()
        .map(|line| {
            Simd::gather_or(
                line.as_bytes(),
                Simd::from_array([11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 12, 13, 14, 15]),
                Simd::splat(b'0'),
            )
            .lanes_eq(Simd::splat(b'1'))
        })
        .collect()
}

pub fn get_gamma_epsilon(input: impl AsRef<[mask8x16]>) -> (u64, u64) {
    let sum = input
        .as_ref()
        .iter()
        .cloned()
        .map(|m| m.select(i8x16::splat(1), i8x16::splat(-1)))
        .sum::<i8x16>();

    let n = mask8x16_to_u16(sum.is_positive());
    let gamma = n as u64 & 0b111111111111;
    let epsilon = (!n) as u64 & 0b111111111111;

    (gamma, epsilon)
}

pub fn part_1(input: impl AsRef<[mask8x16]>) -> u64 {
    let (gamma, epsilon) = get_gamma_epsilon(input);

    gamma * epsilon
}

fn select_rating(input: &mut Vec<u16>, max: bool) -> u16 {
    let mut fallback = 0;
    let mut mask = 0b100000000000u16;
    while input.len() > 1 && mask != 0 {
        let ft = input
            .iter()
            .cloned()
            .map(|n| n & mask != 0)
            .map(|n| n == max)
            .fold((0, 0), |(f, t), n| (f + !n as usize, t + n as usize));
        
        let check = match ft {
            (f, t) if t < f => 0,
            (f, t) if t > f => mask,
            _ if max => mask,
            _ => 0,
        };

        fallback += check;

        input.retain(|n| *n & mask == check);
        mask >>= 1;
    }

    input.first().cloned().unwrap_or(fallback)
}

pub fn part_2(input: impl AsRef<[mask8x16]>) -> u64 {
    let input = input.as_ref();

    let input_u16 = input
        .iter()
        .cloned()
        .map(mask8x16_to_u16)
        .collect::<Vec<_>>();
    
    let (mut input_u16_o2, mut input_u16_co2) = (input_u16.clone(), input_u16);

    let rating_o2 = select_rating(&mut input_u16_o2, true) as u64;
    let rating_co2 = select_rating(&mut input_u16_co2, false) as u64;

    rating_o2 * rating_co2
}
