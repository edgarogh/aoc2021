pub fn generator(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .map(|first_line| first_line.split(',').map(|n| n.parse().unwrap()).collect())
        .unwrap_or_default()
}

pub fn tick(lanterns: &mut Vec<u8>) {
    let mut babies = 0;

    for lantern in lanterns.iter_mut() {
        if *lantern == 0 {
            *lantern = 6;
            babies += 1;
        } else {
            *lantern -= 1;
        }
    }

    lanterns.extend(std::iter::repeat(8).take(babies));
}

fn simulate_for(input: &Vec<u8>, days: usize) -> usize {
    let mut lanterns = [0usize; 9];

    for day in input {
        lanterns[*day as usize] += 1;
    }

    for _ in 0..days {
        let l = lanterns[7];
        lanterns[7] = lanterns[8];
        lanterns[8] = lanterns[0];
        lanterns[0..7].rotate_left(1);
        lanterns[6] += l;
    }

    lanterns.into_iter().sum()
}

pub fn part_1(input: &Vec<u8>) -> usize {
    simulate_for(input, 80)
}

pub fn part_2(input: &Vec<u8>) -> usize {
    simulate_for(input, 256)
}
