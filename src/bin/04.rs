advent_of_code::solution!(4);

pub fn get_winning_and_playable_numbers(game: &str) -> (Vec<u32>, Vec<u32>) {
    // Discard the characters before the colon
    let mut iter = game.split(": ").skip(1).next().unwrap().chars();
    // All the numbers before the | are the winning numbers
    let winning_numbers_str = iter.by_ref().take_while(|c| c != &'|').collect::<String>();
    // The rest of the line is the ticket numbers
    let ticket_numbers_str = iter.collect::<String>();
    let mut winning_numbers: Vec<&str> = winning_numbers_str.split(' ').collect();
    // Remove any empty strings
    winning_numbers.retain(|&x| x != "");
    let mut ticket_numbers: Vec<&str> = ticket_numbers_str.split(' ').collect();
    // Remove any empty strings
    ticket_numbers.retain(|&x| x != "");
    // Convert the winning numbers to integers
    let winning_numbers: Vec<u32> = winning_numbers
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect();
    // Convert the ticket numbers to integers
    let ticket_numbers: Vec<u32> = ticket_numbers
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect();
    return (winning_numbers, ticket_numbers);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (winning_numbers, ticket_numbers) = get_winning_and_playable_numbers(line);
        let mut card_value = 0;
        for number in ticket_numbers {
            if winning_numbers.contains(&number) {
                if card_value == 0 {
                    card_value = 1;
                    continue;
                }
                card_value *= 2;
            }
        }
        sum += card_value;
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games_count: Vec<u32> = Vec::new();
    // Create an array equal to the number of games and set the initail value to 0
    for _ in input.lines() {
        games_count.push(0);
    }

    for (game_number, line) in input.lines().enumerate() {
        let (winning_numbers, ticket_numbers) = get_winning_and_playable_numbers(line);
        let mut winners_count = 0;
        for number in ticket_numbers {
            if winning_numbers.contains(&number) {
                winners_count += 1;
            }
        }
        games_count[game_number] += 1;
        if winners_count == 0 {
            continue;
        }
        for _ in 0..games_count[game_number] {
            for i in 0..winners_count {
                games_count[game_number + i + 1] += 1;
            }
        }
    }
    // Sum all the game counts together
    let mut sum = 0;
    for count in games_count {
        sum += count;
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
