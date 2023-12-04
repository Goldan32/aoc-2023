advent_of_code::solution!(4);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut points: Option<u32> = None;
        let (_, winning_nums_as_string, my_nums_as_string) = line.split([':', '|']).collect_tuple().unwrap();
        let winning_nums: Vec<&str> = winning_nums_as_string.split_whitespace().collect();
        let my_nums: Vec<&str> = my_nums_as_string.split_whitespace().collect();
        for my_num in my_nums {
            if winning_nums.contains(&my_num) {
                points = match points {
                    Some(p) => Some(2*p),
                    None => Some(1)
                }
            }
        }
        sum += points.unwrap_or(0);
    }
    Some(sum)
}

use std::collections::HashMap;

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut idx: usize = 1;
    let mut h: HashMap<usize, u32> = HashMap::new();
    for line in input.lines() {
        let (_, winning_nums_as_string, my_nums_as_string) = line.split([':', '|']).collect_tuple().unwrap();
        let winning_nums: Vec<&str> = winning_nums_as_string.split_whitespace().collect();
        let my_nums: Vec<&str> = my_nums_as_string.split_whitespace().collect();
        let mut points: u32 = 0;
        for my_num in my_nums {
            if winning_nums.contains(&my_num) {
                points += 1;
            }
        }
        let current_count = h.entry(idx).or_insert(1).clone();
        for i in (idx+1)..(idx+TryInto::<usize>::try_into(points).unwrap()+1) {
            h.entry(i).and_modify(|count: &mut u32| *count += current_count).or_insert(current_count + 1);
        }
        idx += 1;
    }
    for v in h.values() {
        sum += *v ;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
