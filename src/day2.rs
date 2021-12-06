use super::common;

pub fn run() {
    let lines = common::file_to_vec("./data/day2/input.txt").unwrap();
    let (horizontal, depth) = pilot_submarine(&lines);
    println!("Day1: part 1 = horizontal: {}, depth: {} => {}", horizontal, depth, horizontal * depth);
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct Movement {
    direction: Direction,
    distance: u32,
}

fn parse_instruction(instruction: String) -> Movement {
    let mut iterator = instruction.splitn(2, ' ');
    let direction_str = iterator.next().unwrap();
    let direction: Direction;
    match direction_str {
        "forward" => direction = Direction::Forward,
        "down" => direction = Direction::Down,
        "up" => direction = Direction::Up,
        _ => panic!("Unknown direction"),
    }
    let distance: u32 = iterator.next().unwrap().parse().unwrap();
    Movement {
        direction,
        distance,
    }
}

fn apply_instruction(postition: (u32, u32), instruction: Movement) -> (u32, u32) {
    let (mut horizontal, mut depth) = postition;
    match instruction.direction {
        Direction::Forward => horizontal += instruction.distance,
        Direction::Down => depth += instruction.distance,
        Direction::Up => depth -= instruction.distance,
    }
    (horizontal, depth)
}

#[allow(dead_code)]
fn pilot_submarine(instructions: &Vec<String>) -> (u32, u32) {
    let starting_depth: u32 = 0;
    let starting_horizontal: u32 = 0;
    let mut position = (starting_horizontal, starting_depth);
    for instruction in instructions.iter() {
        let (horizontal, depth) = position;
        position = apply_instruction(
            (horizontal, depth),
            parse_instruction(instruction.to_string()),
        );
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction_splits_out_distance() {
        let instruction: String = "forward 5".to_string();
        let movement = parse_instruction(instruction);
        let expected_distance: u32 = 5;
        assert_eq!(movement.distance, expected_distance);
    }

    #[test]
    fn test_parse_instruction_splits_out_forward_direction() {
        let instruction: String = "forward 5".to_string();
        let movement = parse_instruction(instruction);
        let expected_direction = Direction::Forward;
        assert_eq!(movement.direction, expected_direction);
    }

    #[test]
    fn test_parse_instruction_splits_out_down_direction() {
        let instruction: String = "down 5".to_string();
        let movement = parse_instruction(instruction);
        let expected_direction = Direction::Down;
        assert_eq!(movement.direction, expected_direction);
    }

    #[test]
    fn test_parse_instruction_splits_out_up_direction() {
        let instruction: String = "up 5".to_string();
        let movement = parse_instruction(instruction);
        let expected_direction = Direction::Up;
        assert_eq!(movement.direction, expected_direction);
    }

    #[test]
    fn test_apply_instruction_with_forward_increases_horizontal() {
        let horizontal: u32 = 0;
        let depth: u32 = 0;
        let position = (horizontal, depth);
        let direction = Direction::Forward;
        let distance: u32 = 5;
        let instruction = Movement {
            direction,
            distance,
        };
        assert_eq!(
            apply_instruction(position, instruction),
            (horizontal + distance, depth)
        );
    }

    #[test]
    fn test_apply_instruction_with_down_increases_depth() {
        let horizontal: u32 = 0;
        let depth: u32 = 0;
        let position = (horizontal, depth);
        let direction = Direction::Down;
        let distance: u32 = 5;
        let instruction = Movement {
            direction,
            distance,
        };
        assert_eq!(
            apply_instruction(position, instruction),
            (horizontal, depth + distance)
        );
    }

    #[test]
    fn test_apply_instruction_with_up_decreases_depth() {
        let horizontal: u32 = 0;
        let depth: u32 = 10;
        let position = (horizontal, depth);
        let direction = Direction::Up;
        let distance: u32 = 5;
        let instruction = Movement {
            direction,
            distance,
        };
        assert_eq!(
            apply_instruction(position, instruction),
            (horizontal, depth - distance)
        );
    }

    #[test]
    fn test_pilot_submarine_with_no_items_returns_zeros() {
        let instructions: Vec<String> = vec![];
        assert_eq!(pilot_submarine(&instructions), (0, 0));
    }

    #[test]
    fn test_pilot_submarine_with_example_returns_expected_position() {
        let instructions: Vec<String> = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        assert_eq!(pilot_submarine(&instructions), (15, 10));
    }
}
