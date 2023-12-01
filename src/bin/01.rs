advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let first = line.find(|c: char| {c.is_digit(10)}).unwrap();
        let last = line.rfind(|c: char| {c.is_digit(10)}).unwrap();
        sum += format!("{}{}", line.as_bytes()[first] as char, line.as_bytes()[last] as char).parse::<u32>().unwrap();

    }
    Some(sum)
}

use phf::phf_map;

const DIGIT_NAMES: phf::Map<&str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let minidx = line.find(|c: char| {c.is_digit(10)});
        let first_digit = match minidx {
            Some(i) => Some((line.as_bytes()[i] as char).to_digit(10).unwrap() as u32),
            None => None
        };

        let maxidx = line.rfind(|c: char| {c.is_digit(10)});
        let last_digit = match maxidx {
            Some(i) => Some((line.as_bytes()[i] as char).to_digit(10).unwrap() as u32),
            None => None
        };

        let mut minidx_str: Option<usize> = None;
        let mut first_str_digit: Option<u32> = None;
        let mut maxidx_str: Option<usize> = None;
        let mut last_str_digit: Option<u32> = None;
        for (key, value) in DIGIT_NAMES.entries() {
            match (line.find(key), minidx_str) {
                (Some(idx), Some(f)) => {
                    if idx < f {
                        minidx_str = Some(idx);
                        first_str_digit = Some(*value);
                    }
                },
                (Some(idx), None) => {
                    minidx_str = Some(idx);
                    first_str_digit = Some(*value);
                }
                (_, _) => ()
            }
            match (line.rfind(key), maxidx_str) {
                (Some(idx), Some(f)) => {
                    if idx > f {
                        maxidx_str = Some(idx);
                        last_str_digit = Some(*value);
                    }
                },
                (Some(idx), None) => {
                    maxidx_str = Some(idx);
                    last_str_digit = Some(*value);
                }
                (_, _) => ()
            }
        }

        let first = match (minidx, minidx_str) {
            (Some(i), Some(j)) => {
                if i < j {
                    first_digit.unwrap()
                } else {
                    first_str_digit.unwrap()
                }
            }
            (Some(_), None) => first_digit.unwrap(),
            (None, Some(_)) => first_str_digit.unwrap(),
            (_, _) => panic!("Bad puzzle input")
        };

        let last = match (maxidx, maxidx_str) {
            (Some(i), Some(j)) => {
                if i > j {
                    last_digit.unwrap()
                } else {
                    last_str_digit.unwrap()
                }
            }
            (Some(_), None) => last_digit.unwrap(),
            (None, Some(_)) => last_str_digit.unwrap(),
            (_, _) => panic!("Bad puzzle input")
        };

        sum += first * 10 + last;
    }
    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
