advent_of_code::solution!(3);

pub fn has_symbol_adjacent(
    coordinates: [usize; 2],
    map: Vec<Vec<char>>,
    row_size: usize,
    symbols: Vec<char>,
) -> (bool, char, char) {
    let c = map[coordinates[0]][coordinates[1]];
    let y = coordinates[0];
    let x = coordinates[1];
    // Check if the row above and below exist
    let row_above = y > 0;
    let row_below = y + 1 < map.len();
    let col_left = x > 0;
    let col_right = x + 1 < row_size;
    let left = if col_left { map[y][x - 1] } else { '.' };
    let left_up = if col_left && row_above {
        map[y - 1][x - 1]
    } else {
        '.'
    };
    let up = if row_above { map[y - 1][x] } else { '.' };
    let right_up = if col_right && row_above {
        map[y - 1][x + 1]
    } else {
        '.'
    };
    let right = if col_right { map[y][x + 1] } else { '.' };
    let right_down = if col_right && row_below {
        map[y + 1][x + 1]
    } else {
        '.'
    };
    let down = if row_below { map[y + 1][x] } else { '.' };
    let left_down = if col_left && row_below {
        map[y + 1][x - 1]
    } else {
        '.'
    };
    let symbols_are_adjacent = symbols.contains(&left)
        || symbols.contains(&left_up)
        || symbols.contains(&up)
        || symbols.contains(&right_up)
        || symbols.contains(&right)
        || symbols.contains(&right_down)
        || symbols.contains(&down)
        || symbols.contains(&left_down);
    return (symbols_are_adjacent, left, right);
}

pub fn generate_full_number(
    coordinates: [usize; 2],
    left: char,
    right: char,
    map: Vec<Vec<char>>,
    row_size: usize,
) -> u32 {
    let y = coordinates[0];
    let x = coordinates[1];
    let c = map[y][x];
    let mut number = c.to_string();

    if left.is_digit(10) {
        let mut i = x;
        while i > 0 {
            i = i - 1;
            if map[y][i].is_digit(10) {
                let next_num = map[y][i].to_string();
                number = next_num + &number;
            } else {
                break;
            }
        }
    }
    if right.is_digit(10) {
        let mut i = x;
        while i < row_size - 1 {
            i = i + 1;
            if map[y][i].is_digit(10) {
                let next_num = map[y][i].to_string();
                number = number + &next_num;
            } else {
                break;
            }
        }
    }
    return number.parse::<u32>().unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbols = vec![
        '+', '-', '*', '/', '%', '#', '@', '!', '?', '&', '$', '^', '=',
    ];
    // Create an array of arrays from the input lines
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Find any numbers that are adjacent to a symbol
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        let mut working_number: u32 = 0;
        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                let (is_adjacent, left, right) =
                    has_symbol_adjacent([y, x], map.clone(), row.len(), symbols.clone());
                if is_adjacent {
                    let parsed_number =
                        generate_full_number([y, x], left, right, map.clone(), row.len());
                    if working_number != parsed_number {
                        sum = sum + parsed_number;
                        working_number = parsed_number;
                    }
                }
            } else {
                working_number = 0;
            }
        }
    }
    return Some(sum);
}

pub fn get_number_adjacent(coordinates: [usize; 2], map: Vec<Vec<char>>) -> Vec<[usize; 2]> {
    let y = coordinates[0];
    let x = coordinates[1];
    // Check if the row above and below exist
    let row_above = y > 0;
    let row_below = y + 1 < map.len();
    let col_left = x > 0;
    let col_right = x + 1 < map[y].len();
    let left = if col_left { map[y][x - 1] } else { '.' };
    let left_up = if col_left && row_above {
        map[y - 1][x - 1]
    } else {
        '.'
    };
    let up = if row_above { map[y - 1][x] } else { '.' };
    let right_up = if col_right && row_above {
        map[y - 1][x + 1]
    } else {
        '.'
    };
    let right = if col_right { map[y][x + 1] } else { '.' };
    let right_down = if col_right && row_below {
        map[y + 1][x + 1]
    } else {
        '.'
    };
    let down = if row_below { map[y + 1][x] } else { '.' };
    let left_down = if col_left && row_below {
        map[y + 1][x - 1]
    } else {
        '.'
    };
    let mut adjacent_coordinates = Vec::new();
    if left.is_digit(10) {
        adjacent_coordinates.push([y, x - 1]);
    }
    if left_up.is_digit(10) {
        adjacent_coordinates.push([y - 1, x - 1]);
    }
    if up.is_digit(10) {
        adjacent_coordinates.push([y - 1, x]);
    }
    if right_up.is_digit(10) {
        adjacent_coordinates.push([y - 1, x + 1]);
    }
    if right.is_digit(10) {
        adjacent_coordinates.push([y, x + 1]);
    }
    if right_down.is_digit(10) {
        adjacent_coordinates.push([y + 1, x + 1]);
    }
    if down.is_digit(10) {
        adjacent_coordinates.push([y + 1, x]);
    }
    if left_down.is_digit(10) {
        adjacent_coordinates.push([y + 1, x - 1]);
    }
    return adjacent_coordinates;
}

pub fn part_two(input: &str) -> Option<u32> {
    // Create an array of arrays from the input lines
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Find any numbers that are adjacent to a symbol
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'*' {
                // Find the adjacent numbers
                let adjacent_coordinates = get_number_adjacent([y, x], map.clone());
                if adjacent_coordinates.len() > 1 {
                    let mut adjacent_numbers = Vec::new();
                    for coordinates in adjacent_coordinates {
                        let left = if coordinates[1] - 1 > 0 {
                            map[coordinates[0]][coordinates[1] - 1]
                        } else {
                            '.'
                        };
                        let right = if coordinates[1] + 1 < map.len() {
                            map[coordinates[0]][coordinates[1] + 1]
                        } else {
                            '.'
                        };
                        let parsed_number =
                            generate_full_number(coordinates, left, right, map.clone(), row.len());
                        adjacent_numbers.push(parsed_number);
                    }
                    // Remove duplicates
                    adjacent_numbers.sort();
                    adjacent_numbers.dedup();
                    if adjacent_numbers.len() == 2 {
                        // Multiply the numbers together
                        let mut product = 1;
                        for number in adjacent_numbers {
                            product = product * number;
                        }
                        sum = sum + product;
                    }
                }
            }
        }
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
