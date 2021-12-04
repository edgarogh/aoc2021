use std::simd::{i8x16, mask16x16, mask8x16, simd_swizzle, u16x16};

fn u16_to_mask8x16(mask: u16) -> mask8x16 {
    let lsb0 = (mask & 0xff) as i8;
    let lsb1 = (mask >> 8) as i8;
    let mask = i8x16::from_array([lsb0, lsb1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mask = simd_swizzle!(mask, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]);

    let mask = mask
        | i8x16::from_array([
            !0x01, !0x02, !0x04, !0x08, !0x10, !0x20, !0x40, !-0x80, !0x01, !0x02, !0x04, !0x08,
            !0x10, !0x20, !0x40, !-0x80,
        ]);

    mask.lanes_eq(i8x16::splat(-1i8))
}

fn mask8x16_to_u16(mask: mask8x16) -> u16 {
    let mask_shifted = mask16x16::from(mask).select(u16x16::splat(1), u16x16::splat(0))
        << u16x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    mask_shifted.horizontal_sum()
}

pub fn generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn get_gamma_epsilon(input: impl AsRef<[u16]>) -> (u64, u64) {
    let sum = input
        .as_ref()
        .iter()
        .cloned()
        .map(u16_to_mask8x16)
        .map(|m| m.select(i8x16::splat(1), i8x16::splat(-1)))
        .sum::<i8x16>();

    let n = mask8x16_to_u16(sum.is_positive());
    let gamma = n as u64 & 0b111111111111;
    let epsilon = (!n) as u64 & 0b111111111111;

    (gamma, epsilon)
}

pub fn part_1(input: impl AsRef<[u16]>) -> u64 {
    let (gamma, epsilon) = get_gamma_epsilon(input);

    gamma * epsilon
}
