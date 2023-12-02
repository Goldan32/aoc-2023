advent_of_code::solution!(2);

use phf::phf_map;
use std::collections::HashMap;

static COLORS: phf::Map<&str, u32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    for line in input.lines() {
        let mut ok = true;
        let mut v: Vec<&str> = line.split([':', ';', ',']).collect::<Vec<&str>>();
        let id: u32 = v[0].trim_start_matches(|c: char| {!c.is_digit(10)}).parse().unwrap();
        v.remove(0);
        for item in v {
            let (num, color) = item.trim().split_once(' ').unwrap();
            if *(COLORS.get(color).unwrap()) < num.parse::<u32>().unwrap() {
                ok = false;
                break;
            }
        };
        if ok { sum += id; }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    for line in input.lines() {
        let mut max: HashMap<&str, u32> = HashMap::new();
        let mut v: Vec<&str> = line.split([':', ';', ',']).collect::<Vec<&str>>();
        v.remove(0);
        for item in v {
            let (numstr, color) = item.trim().split_once(' ').unwrap();
            let num = numstr.parse::<u32>().unwrap();
            max.entry(color)
                .and_modify(|m| {if *m < num { *m = num; } })
                .or_insert(num);
        };
        sum += max.values().product::<u32>();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
