use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut rays: HashSet<usize> = HashSet::new();
    rays.insert(input.lines().next().unwrap().find(|c| c == 'S').unwrap());
    let mut split_count = 0;
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars()
            .map(|c| c == '^')
            .collect())
        .collect();
    map.iter()
        .for_each(|line| {
            line
                .iter()
                .enumerate()
                .for_each(|(i, node)| {
                    if *node && rays.contains(&i) {
                        rays.remove(&i);
                        rays.insert(i+1) as usize;
                        rays.insert(i-1) as usize;
                        split_count += 1;
                    }
                });

        });
        Some(split_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
