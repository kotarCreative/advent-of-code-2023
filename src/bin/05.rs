advent_of_code::solution!(5);

pub fn input_generator(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    let mut map_index: isize = -1;
    for line in input.lines() {
        if regex::Regex::new(r"^seeds").unwrap().is_match(line) {
            seeds = line
                .split_once(": ")
                .unwrap()
                .1
                .split(" ")
                .map(|x| x.trim().parse().unwrap())
                .collect();
            continue;
        }
        if line.is_empty() {
            continue;
        }

        if line.contains("map") {
            map_index += 1;
            maps.push(Vec::new());
            continue;
        }

        let map: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        maps[map_index as usize].push(map);
    }
    return (seeds, maps);
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = input_generator(input);
    let mut seed_locations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut current_location = seed;
        for options in maps.iter() {
            for option in options.iter() {
                let bounds = option[1]..=(option[1] + option[2] - 1);
                if bounds.contains(&current_location) {
                    // The location is in the map so we can calculate its destination
                    let spread = current_location - option[1];
                    current_location = option[0] + spread;
                    // If we found a location we can move to the next map
                    break;
                } else {
                    // The location is not in the map so it will be the same
                    continue;
                }
            }
        }
        seed_locations.push(current_location);
    }
    return Some(*seed_locations.iter().min().unwrap());
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = input_generator(input);
    let mut lowest_seed_location = 0;
    // Convert the seeds into their appropriate ranges
    let all_seed_ranges = seeds.chunks(2).map(|chunk| chunk[0]..=(chunk[0] + chunk[1])).collect::<Vec<_>>();
    for seed in 0..=u64::MAX {
        let mut current_location = seed;
        for options in maps.iter().rev() {
            for option in options.iter() {
                if current_location >= option[0] && current_location <= option[0] + option[2] - 1 {
                    // The location is in the map so we can calculate its destination
                    let spread = current_location - option[0];
                    current_location = option[1] + spread;
                    // If we found a location we can move to the next map
                    break;
                } else {
                    // The location is not in the map so it will be the same
                    continue;
                }
            }
        }
        if all_seed_ranges.iter().any(|range| range.contains(&current_location)) {
            lowest_seed_location = seed;
            break;
        }
    }
    return Some(lowest_seed_location);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
