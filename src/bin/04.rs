use md5::compute;

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_hash(input, 5))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_hash(input, 6))
}

fn find_hash(key: &str, zeroes: u32) -> u32 {
    let mut i = 0;
    let mut zero_string = "".to_owned();
    for _ in 0..zeroes {
        zero_string.push_str("0");
    }
    loop {
        let mut input_string = key.to_owned();
        input_string.push_str(&*i.to_string());
        let hash = format!("{:x}", compute(input_string));
        if hash.starts_with(&zero_string) {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
