use regex::Regex;

advent_of_code::solution!(1);

// Extract the first and last numbers from each line of the input.
pub fn part_one(input: &str) -> Option<u32> {
    let re2 = Regex::new(r"(\d)").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        // Extract the first and last numbers from the line.
        let mut numbers = re2.captures_iter(line);
        let mut number_str = String::new();
        let first_number = numbers.next().unwrap().get(0).unwrap().as_str();
        number_str.push_str(first_number);
        let last_number = numbers.last();
        if last_number.is_none() {
            number_str.push_str(first_number);
        } else {
            number_str.push_str(last_number?.get(0).unwrap().as_str());
        }
        // Convert the string to a number.
        let number = number_str.parse::<u32>().unwrap();
        sum += number;
    }
    return Some(sum);
}

pub fn convert_number_string_to_number(number_str: &str) -> String {
    // Convert the spelled out number to a number.
    let number = match number_str.to_lowercase().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => number_str.parse::<u32>().unwrap(),
    };
    let num: String = number.to_string();
    return num;
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_re = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        // Extract the first and last numbers from the line.
        let mut numbers = re.captures_iter(line);
        let reverse_line = line.chars().rev().collect::<String>();
        let mut reverse_numbers = reverse_re.captures_iter(&reverse_line);
        let mut number_str = String::new();
        let first_number = numbers.next().unwrap().get(0).unwrap().as_str();
        number_str.push_str(&convert_number_string_to_number(first_number));
        let last_number = reverse_numbers
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect::<String>();
        number_str.push_str(&convert_number_string_to_number(&last_number));
        // Convert the string to a number.
        let number = number_str.parse::<u32>().unwrap();
        // println!("{}", number);
        sum += number;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
