use std::str::Lines;

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Lines = input.lines();
    let mut paper_feet: u32 = 0;
    for line in lines {
        paper_feet += get_present_required_paper(line);
    }
    Some(paper_feet)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Lines = input.lines();
    let mut paper_feet: u32 = 0;
    for line in lines {
        paper_feet += get_present_required_ribbon(line);
    }
    Some(paper_feet)
}

fn get_present_required_paper(line: &str) -> u32 {
    let (l, w, h) = parse_present(line);

    //2*l*w + 2*w*h + 2*h*l
    let paper_size = (2 * l * w) + (2 * w * h) + (2 * h * l);

    let mut sides: Vec<u32> = Vec::from([l, w, h]);
    sides.sort();
    let slack = sides[0] * sides[1];
    paper_size + slack
}

fn get_present_required_ribbon(line: &str) -> u32 {
    let (l, w, h) = parse_present(line);

    let mut sides: Vec<u32> = Vec::from([l, w, h]);
    sides.sort();
    sides[0] + sides[0] + sides[1] + sides[1] + (l * w * h)
}

fn parse_present(line: &str) -> (u32, u32, u32) {
    let dims: Vec<u32> = line.split("x").map(|s| s.parse().unwrap()).collect();
    (dims[0], dims[1], dims[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
