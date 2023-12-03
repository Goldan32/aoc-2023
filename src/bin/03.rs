advent_of_code::solution!(3);

#[derive(Debug)]
struct NumberInfo {
    num: u32,
    indices: Vec<usize>,
    line: usize,
}

#[derive(Debug)]
struct SymbolInfo{
    idx: usize,
    line: usize,
}

fn extract_numbers(input: &str, lineidx: usize) -> Vec<NumberInfo> {
    let mut result = Vec::new();
    let mut current_num = None;
    let mut current_indices = Vec::new();

    for (idx, c) in input.char_indices() {
        if c.is_ascii_digit() {
            current_indices.push(idx);
            let digit_value = c.to_digit(10).unwrap();
            current_num = Some(current_num.unwrap_or(0) * 10 + digit_value);
        } else {
            if let Some(num) = current_num {
                result.push(NumberInfo {
                    num,
                    indices: current_indices.clone(),
                    line: lineidx,
                });
                current_num = None;
                current_indices.clear();
            }
        }
    }
    if let Some(num) = current_num {
        result.push(NumberInfo {
            num,
            indices: current_indices,
            line: lineidx,
        });
    }

    result
}

fn extract_symbols(input: &str, lineidx: usize) -> Vec<SymbolInfo> {
    let mut result: Vec<SymbolInfo> = Vec::new();
    let v: Vec<_> = input.match_indices(|c: char| {!c.is_digit(10) && !(c == '.')}).collect();
    for (idx, _) in v {
        result.push(SymbolInfo{idx, line: lineidx});
    }
    result
}

fn extract_gears(input: &str, lineidx: usize) -> Vec<SymbolInfo> {
    let mut result: Vec<SymbolInfo> = Vec::new();
    let v: Vec<_> = input.match_indices(|c: char| {c == '*'}).collect();
    for (idx, _) in v {
        result.push(SymbolInfo{idx, line: lineidx});
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers:Vec<Vec<NumberInfo>> = Vec::new();
    let mut symbols: Vec<Vec<SymbolInfo>> = Vec::new();
    let mut sum: u32 = 0;

    for (lineidx, line) in input.lines().enumerate() {
        numbers.push(extract_numbers(line, lineidx));
        symbols.push(extract_symbols(line, lineidx));

    }
    for num in numbers.iter().flatten() {
        //println!("{:?}", num);
        for sym in symbols.iter().flatten() {
            //println!("{:?}", sym);
            let x = num.indices.iter().map(|n| n.abs_diff(sym.idx)).min().unwrap();
            let y = num.line.abs_diff(sym.line);
            if (x <= 1usize) && (y <= 1usize) {
                sum += num.num;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers:Vec<Vec<NumberInfo>> = Vec::new();
    let mut symbols: Vec<Vec<SymbolInfo>> = Vec::new();
    let mut sum: u32 = 0;

    for (lineidx, line) in input.lines().enumerate() {
        numbers.push(extract_numbers(line, lineidx));
        symbols.push(extract_gears(line, lineidx));

    }
    for sym in symbols.iter().flatten() {
        //println!("{:?}", sym);
        let mut neighbors: Vec<u32> = Vec::new();
        for num in numbers.iter().flatten() {
            //println!("{:?}", num);
            let x = num.indices.iter().map(|n| n.abs_diff(sym.idx)).min().unwrap();
            let y = num.line.abs_diff(sym.line);
            if (x <= 1usize) && (y <= 1usize) {
                neighbors.push(num.num);
            }
        }
        if neighbors.len() == 2 {
            sum += neighbors[0] * neighbors[1];
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
