advent_of_code::solution!(6);

#[derive(Debug)]
pub struct Race {
    time: u64,
    record: u64,
}

// Runs a single race for a given hold time and returns the
// total distance traveled
fn run_race(hold_time: u64, total_time: u64) -> u64 {
    let speed = hold_time;
    let remaining_time = total_time - hold_time;
    return speed * remaining_time;
}

impl Race {
    pub fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    pub fn total_winning_races(&self) -> u64 {
        let mut winning_races = 0;
        for hold_time in 1..=self.time {
            let distance = run_race(hold_time, self.time);
            if distance > self.record {
                winning_races += 1;
            }
        }
        return winning_races;
    }
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u64>>()
}

// Removes all white space from the string and returns a single number
fn merge_numbers(input: &str) -> u64 {
   input.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().unwrap()
}

pub fn input_generator(input: &str) -> Vec<Race> {
    let (times, records) = input.split_once('\n').unwrap();
    let times = parse_numbers(times.split(":").nth(1).unwrap());
    let records = parse_numbers(records.split(":").nth(1).unwrap());

    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race::new(times[i], records[i]));
    }
    return races;
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = input_generator(input);
    return Some(races.iter().map(|race| race.total_winning_races()).product());
}

pub fn part_two(input: &str) -> Option<u64> {
   let (times, records) = input.split_once('\n').unwrap();
   let time = merge_numbers(times.split(":").nth(1).unwrap());
   let record = merge_numbers(records.split(":").nth(1).unwrap());

   let race = Race::new(time, record);
   return Some(race.total_winning_races());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
