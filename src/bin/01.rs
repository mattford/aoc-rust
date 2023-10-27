pub fn part_one(input: &str) -> Option<i32> {
    let mut floor: i32 = 0;
    let parts: Vec<&str> = input.split("").collect();
    for part in parts {
        match part {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => ()
        }
    }
    Some(floor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut floor: i32 = 0;
    let parts: Vec<&str> = input.split("").collect();
    for (pos, &part) in parts.iter().enumerate() {
        match part {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => ()
        }
        if floor < 0 {
            return Some(pos as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
