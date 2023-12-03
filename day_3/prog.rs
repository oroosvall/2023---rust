
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn is_symbol2(ch: char) -> bool 
{
    if ch.is_digit(10) || ch == '.' {
        return false;
    }
    true
}

fn part_1(lines :&Vec<String>) -> i64
{
    let mut sum = 0;
    let line_count = lines.len();
    for y in 0..line_count {
        let line = &lines[y];
        let chars : Vec<char> = line.chars().collect();
        let char_count = chars.len();

        let mut x : i32 = 0;
        while (x as usize) < char_count {
            let mut nr_len : i32 = 0;
            if chars[x as usize].is_digit(10) {
                while (x as usize) < char_count && chars[x as usize].is_digit(10) {
                    nr_len += 1;
                    x += 1;
                }
                let mut add = false;
                for i in 0.max(y as i32 - 1)..(line_count as i32).min(y as i32+ 2) {
                    for c in 0.max(x - nr_len - 1)..(char_count as i32).min(x as i32 + 1) {
                        let ch = lines[i as usize].chars().nth(c as usize).unwrap();
                        if is_symbol2(ch) {
                            add = true;
                        }
                    }
                }
                if add {
                    match line[(x-nr_len) as usize..(x as usize)].parse::<i64>() {
                        Ok(nr) => { sum += nr },
                        Err(_) => {},
                    }
                }
            } else {
                x += 1;
            }
        }
    }
    sum
}

fn get_number(lines :&Vec<String>, y :usize, x :usize, max_x :usize ) -> (i64, bool) {
    let line = &lines[y];
    let chars : Vec<char> = line.chars().collect();
    let start_ch = chars[x];
    if start_ch.is_digit(10) {
        let mut start_x = x;
        // find start and get number
        while start_x > 0 && chars[start_x].is_digit(10) {
            start_x -= 1;
        }
        if start_x != 0 || !chars[start_x].is_digit(10) {
            start_x += 1;
        }
        let mut end_x = x;
        while end_x < max_x && chars[end_x].is_digit(10) {
            end_x += 1;
        }
        return (line[start_x..end_x].parse::<i64>().unwrap(), true);
    } 
    (0, false)
}

fn part_2(lines :&Vec<String> ) -> i64
{
    let mut sum = 0;
    let line_count = lines.len();
    for y in 0..line_count {
        let line = &lines[y];
        let chars : Vec<char> = line.chars().collect();
        let char_count = chars.len();

        let mut x : i32 = 0;
        while (x as usize) < char_count {
            if chars[x as usize] == '*' {
                let mut nums : Vec<i64> = vec!();

                for i in 0.max(y as i32 - 1)..(line_count as i32).min(y as i32 + 2) {
                    for c in 0.max(x - 1)..(char_count as i32).min(x as i32 + 2) {
                        let (num, found) = get_number(lines, i as usize, c as usize, char_count);
                        if found {
                            if !nums.contains(&num) {
                                nums.push(num);
                            }
                        }
                    }
                }

                if nums.len() == 2 {
                    sum += nums.into_iter().product::<i64>();
                }
                x += 1;
            }
            else {
                x += 1;
            }
        }
    }
    sum
}

fn main() -> Result<(), Error>
{
    let mut now = Instant::now();
    let input : Vec<String> = read_to_vec(File::open("input.txt")?)?;
    println!("Read input: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r1 = part_1(&input);
    println!("Part 1: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r2 = part_2(&input);
    println!("Part 2: {} µs", now.elapsed().as_micros());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}