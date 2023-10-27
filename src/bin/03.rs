use std::cmp::min;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let moves: Vec<&str> = input.split("").collect();
    Some(simulate(moves, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves: Vec<&str> = input.split("").collect();
    Some(simulate(moves, 1))
}

fn simulate(moves: Vec<&str>, mut helpers: u32) -> u32 {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();
    helpers += 1;
    let mut coords: Vec<(i32, i32)> = Vec::new();
    for _ in 0..helpers {
        coords.push((0, 0));
    }
    houses.insert((0, 0), helpers);
    for (i, mv) in moves.iter().enumerate() {
        let idx = i as i32;
        let c_idx = min(coords.len() - 1, (idx % 2) as usize);
        match *mv {
            "^" => coords[c_idx].0 -= 1,
            "v" => coords[c_idx].0 += 1,
            "<" => coords[c_idx].1 -= 1,
            ">" => coords[c_idx].1 += 1,
            _ => continue
        }

        let mut current = *houses.get(&coords[c_idx]).unwrap_or(&0);
        current += 1;
        houses.insert(coords[c_idx], current);
    }
    houses.len() as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
