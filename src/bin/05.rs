use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    input.lines().for_each(|line| {
        if is_nice(line) {
            count += 1;
        }
    });
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn is_nice(line: &str) -> bool {
    // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    let vowel_regex = Regex::new("[aeiou]").unwrap();
    let matches: Vec<_> = vowel_regex.find_iter(line).map(|m| m.as_str()).collect();
    if matches.len() < 3 {
        return false;
    }
    // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    let twice_in_a_row_regex = Regex::new("(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();
    let matches: Vec<_> = twice_in_a_row_regex.find_iter(line).map(|m| m.as_str()).collect();
    if matches.len() < 1 {
        return false;
    }
    // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
    let blacklist_regex = Regex::new("(ab|cd|pq|xy)").unwrap();
    !blacklist_regex.is_match(line)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
