#![feature(
    inline_const,
    int_abs_diff,
    maybe_uninit_array_assume_init,
    maybe_uninit_extra,
    portable_simd
)]

mod day1;
mod day2;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;

aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
    day6 : generator => part_1, part_2;
    day7 : generator => part_1, part_2;
    day8 : generator => part_1;
}
