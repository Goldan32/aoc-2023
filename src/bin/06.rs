advent_of_code::solution!(6);

use std::iter::zip;

pub fn part_one(input: &str) -> Option<u32> {
    let input_parsed: Vec<_> = input.split('\n').collect();
    let mut times: Vec<_> = input_parsed[0].split(' ').filter(|x| {!x.is_empty() && (*x != "Time:")}).collect();
    let mut dists: Vec<_> = input_parsed[1].split(' ').filter(|x| {!x.is_empty()&& (*x != "Distance:")}).collect();

    println!("{:?}", times);
    println!("{:?}", dists);

    let mut pro: u32 = 1;

    for (t, d) in zip(times, dists) {
        let time = t.parse::<u32>().unwrap();
        let dist = d.parse::<u32>().unwrap();
        let b = (-1 * time as i32) as f64;
        let c = (dist as i32) as f64;
        let a = 1.0;
        let mut x2 = ((-1.0 * b + (b*b - 4.0*a*c).sqrt()) / (2.0*a)).ceil() as i32;
        x2 -= 1;
        let x1 = ((-1.0 * b - (b*b - 4.0*a*c).sqrt()) / (2.0*a)).floor() as i32;
        println!("{} and {} from {} {} {}", x1, x2, a, b, c);
        pro *= (x2 - x1).abs() as u32;
    }

    Some(pro)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_parsed: Vec<_> = input.split('\n').collect();
    let time = input_parsed[0].chars().filter(|c| {c.is_digit(10)}).collect::<String>().parse::<u32>().unwrap();
    let dist = input_parsed[1].chars().filter(|c| {c.is_digit(10)}).collect::<String>().parse::<u64>().unwrap();
    let b = (-1 * time as i64) as f64;
    let c = (dist as i64) as f64;
    let a = 1.0;
    let mut x2 = ((-1.0 * b + (b*b - 4.0*a*c).sqrt()) / (2.0*a)).ceil() as i64;
    x2 -= 1;
    let x1 = ((-1.0 * b - (b*b - 4.0*a*c).sqrt()) / (2.0*a)).floor() as i64;
    Some((x2-x1) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
