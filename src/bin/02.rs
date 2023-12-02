use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let total_red = 12;
    let total_blue = 14;
    let total_green = 13;
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let mut sum = 0;
    for game in input.lines() {
        let game_num = re_game
            .captures_iter(game)
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let rounds = game.split(";");
        let mut game_is_possible = true;
        for round in rounds {
            let captures = re.captures_iter(round);
            for capture in captures {
                let number = capture.get(1).unwrap().as_str();
                let color = capture.get(2).unwrap().as_str();
                match color {
                    "red" => {
                        if number.parse::<u32>().unwrap() > total_red {
                            game_is_possible = false;
                            break;
                        }
                    }
                    "blue" => {
                        if number.parse::<u32>().unwrap() > total_blue {
                            game_is_possible = false;
                            break;
                        }
                    }
                    "green" => {
                        if number.parse::<u32>().unwrap() > total_green {
                            game_is_possible = false;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            if !game_is_possible {
                break;
            }
        }
        if game_is_possible {
            sum += game_num.parse::<u32>().unwrap();
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let mut sum = 0;
    for game in input.lines() {
        let mut highest_cube_counts = vec![0, 0, 0];
        let rounds = game.split(";");
        for round in rounds {
            let captures = re.captures_iter(round);
            for capture in captures {
                let number = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = capture.get(2).unwrap().as_str();
                match color {
                    "red" => {
                        if highest_cube_counts[0] < number {
                            highest_cube_counts[0] = number;
                        }
                    }
                    "blue" => {
                        if highest_cube_counts[1] < number {
                            highest_cube_counts[1] = number;
                        }
                    }
                    "green" => {
                        if highest_cube_counts[2] < number {
                            highest_cube_counts[2] = number;
                        }
                    }
                    _ => (),
                }
            }
        }
        let highest_counts_power =
            highest_cube_counts[0] * highest_cube_counts[1] * highest_cube_counts[2];
        sum += highest_counts_power;
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
