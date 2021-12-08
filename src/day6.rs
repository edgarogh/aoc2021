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
    std::iter::repeat(())
        .take(days)
        .fold(input.clone(), |mut lanterns, _| {
            tick(&mut lanterns);
            lanterns
        })
        .len()
}

pub fn part_1(input: &Vec<u8>) -> usize {
    simulate_for(input, 80)
}

pub fn part_2(input: &Vec<u8>) -> usize {
    simulate_for(input, 256)
}
