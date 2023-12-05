advent_of_code::solution!(5);

use itertools::Itertools;

#[derive(Debug)]
struct MyRange {
    dst: u64,
    src: u64,
    len: u64,
}

#[derive(Debug)]
struct Map {
    v: Vec<MyRange>
}

impl Map {
    fn get_dst(&self, seed: u64) -> u64 {
        let mut retval: u64 = 0;
        for r in &self.v {
            if (r.src .. (r.src + r.len)).contains(&seed) {
                retval = r.dst + (seed - r.src);
                break;
            }
            retval = seed;
        }
        retval
    }

    fn get_len(&self, seed: u64) -> i64 {
        let mut retval: i64 = 0;
        for r in &self.v {
            if (r.src .. (r.src + r.len)).contains(&seed) {
                retval = r.dst as i64 - r.src as i64;
                break;
            }
        }
        retval
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let fields: Vec<&str> = input.split("\n\n").collect();
    let mut seeds: Vec<u64> = fields[0].trim_start_matches("seeds: ").split(" ").map(|x| { x.parse::<u64>().unwrap() }).collect();
    for i in 1..fields.len() {
        let m: Vec<&str> = fields[i].trim_start_matches(|c: char| { !c.is_digit(10)}).trim().split('\n').collect();
        let mut map = Map { v: Vec::new() };
        for line in m {
            let (dst, src, len) = line.split(' ').map(|x| { x.parse::<u64>().unwrap() }).collect_tuple().unwrap();
            map.v.push(MyRange {dst, src, len});
        }
        for idx in 0..seeds.len() {
            seeds[idx] = map.get_dst(seeds[idx]);
        }
    }
    Some(*seeds.iter().min().unwrap())
}

use std::ops::Range;
use std::collections::HashSet;

pub fn part_two(input: &str) -> Option<u64> {

    let fields: Vec<&str> = input.split("\n\n").collect();
    //println!("{:#?}", fields);
    let seeds: Vec<u64> = fields[0].trim_start_matches("seeds: ").split(" ").map(|x| { x.parse::<u64>().unwrap() }).collect();
    let mut seed_ranges: HashSet<Range<u64>> = HashSet::new();
    let mut si: usize = 0;
    while si < seeds.len() {
        seed_ranges.insert(seeds[si] .. seeds[si+1]+seeds[si]);
        si += 2;
    }
    //println!("{:#?}", seed_ranges);

    for i in 1..fields.len() {
        let map_helper: Vec<&str> = fields[i].trim_start_matches(|c: char| { !c.is_digit(10)}).trim().split('\n').collect();
        let mut map = Map { v: Vec::new() };
        for line in map_helper {
            let (dst, src, len) = line.split(' ').map(|x| { x.parse::<u64>().unwrap() }).collect_tuple().unwrap();
            map.v.push(MyRange {dst, src, len});
        }

        let mut new_seed_ranges: HashSet<Range<u64>> = seed_ranges.clone();
        for sr in &seed_ranges {
            for m in &map.v {
                if sr.contains(&m.src) {
                    new_seed_ranges.insert(sr.start.clone() .. m.src.clone());
                    new_seed_ranges.insert(m.src.clone() .. sr.end.clone());
                    new_seed_ranges.remove(&(sr.start.clone() .. sr.end.clone()));
                }
            }
        }

        //println!("{:#?}", seed_ranges);
        //println!("{:#?}", new_seed_ranges);

        seed_ranges = new_seed_ranges;

        let mut even_newer_seed_ranges: HashSet<Range<u64>> = HashSet::new();

        for sr in &seed_ranges {
            let len = map.get_len(sr.start);
            //println!("{:#?}", len);
            even_newer_seed_ranges.insert((sr.start as i64 + len) as u64 .. (sr.end as i64 + len) as u64);
        }

        seed_ranges = even_newer_seed_ranges;

        println!("Still running...");
    }

    //println!("{:#?}", seed_ranges);

    let mut min_loc = u64::MAX;
    for r in seed_ranges {
        if r.start < min_loc && r.start != 0 {
            min_loc = r.start;
        }
    }

    //println!("{min_loc}");

    Some(min_loc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
